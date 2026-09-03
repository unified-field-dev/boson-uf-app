//! Auth helpers and task summary builders for Boson server functions.
//!
//! Pure DTO mappers and aggregates live in [`boson_backend`].

#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;

#[cfg(feature = "ssr")]
pub(super) use boson_backend::{
    apply_task_config_update, job_to_summary, parse_job_status_filter, run_to_summary,
    task_config_to_dto, task_summary_from_parts, TaskStatsAgg,
};

#[cfg(feature = "ssr")]
pub(super) fn boson_backend(
) -> Result<std::sync::Arc<dyn boson_coordinator::BosonCoordinatorBackend>, ServerFnError> {
    leptos::context::use_context::<std::sync::Arc<dyn boson_coordinator::BosonCoordinatorBackend>>()
        .ok_or_else(|| ServerFnError::new("Boson backend not in request context"))
}

/// Require an authenticated session (`SessionSnapshot` / `session_user_id`).
///
/// `SessionSnapshot` does not carry `email_verified`; use
/// [`require_email_verified`] for the task-config UI gate.
#[cfg(feature = "ssr")]
pub(super) fn require_session(ctx: &higgs::Higgs) -> Result<(), ServerFnError> {
    if ctx.session_user_id().is_some() {
        Ok(())
    } else {
        Err(ServerFnError::new(
            "Authentication is required for this action",
        ))
    }
}

/// Mirror the task-config UI `requires_email_verification` gate server-side.
///
/// Uses axum-login's auth user (via lepton-auth) because `SessionSnapshot`
/// only stores `user_id` + `auth_hash`. Lab hosts with the `e2e-lab` feature may
/// force the outcome via [`crate::e2e_lab::set_email_verified_override`].
#[cfg(feature = "ssr")]
pub(super) async fn require_email_verified() -> Result<(), ServerFnError> {
    if let Some(verified) = crate::e2e_lab::email_verified_override() {
        return if verified {
            Ok(())
        } else {
            Err(ServerFnError::new(
                "Email verification is required for this action",
            ))
        };
    }
    let user = lepton_auth::extract_auth_user().await?;
    if user.email_verified {
        Ok(())
    } else {
        Err(ServerFnError::new(
            "Email verification is required for this action",
        ))
    }
}

/// Load every registered task summary (sorted) without re-entering a server fn.
#[cfg(feature = "ssr")]
pub(super) async fn load_all_task_summaries(
    backend: &dyn boson_coordinator::BosonCoordinatorBackend,
) -> Result<Vec<super::types::TaskSummary>, ServerFnError> {
    let registry = backend.registry();
    let mut tasks = Vec::with_capacity(registry.len());
    for desc in registry.iter() {
        tasks.push(build_task_summary(backend, desc).await?);
    }
    boson_backend::sort_tasks_by_name(&mut tasks);
    Ok(tasks)
}

/// Build a single task summary via registry descriptor and backend stats.
///
/// Fails closed when task config cannot be loaded (no silent defaults).
#[cfg(feature = "ssr")]
pub(super) async fn build_task_summary(
    backend: &dyn boson_coordinator::BosonCoordinatorBackend,
    desc: &boson_runtime::TaskDescriptor,
) -> Result<super::types::TaskSummary, ServerFnError> {
    use boson_core::JobStatus;

    let name = desc.name.to_string();
    let config = backend
        .get_task_config(&name)
        .await
        .map_err(|e| ServerFnError::new(boson_backend::format_task_config_load_error(e)))?;

    let jobs_queued = u32::try_from(
        backend
            .count_jobs_for_task(&name, Some(JobStatus::Queued))
            .await,
    )
    .unwrap_or(u32::MAX);
    let run_stats = backend.task_run_stats(&name).await;

    let stats = TaskStatsAgg {
        jobs_queued,
        runs_total: run_stats.runs_total,
        success_count: run_stats.success_count,
    };

    Ok(task_summary_from_parts(
        desc.name,
        desc.signature_json,
        desc.default_priority,
        desc.default_pool,
        &config,
        stats,
    ))
}

/// Classify a boundary error string for tracing (`error_class` field).
#[cfg(feature = "ssr")]
pub(super) fn error_class(msg: &str) -> &'static str {
    if msg.contains("Authentication is required") || msg.contains("Email verification is required")
    {
        "auth"
    } else if msg.starts_with("Invalid range_secs")
        || msg.starts_with("Invalid task config update")
        || msg.contains("is required")
        || msg.contains("too long")
        || msg.contains("unsafe path")
    {
        "validation"
    } else if msg.contains("not found") || msg.contains("Task config not found") {
        "not_found"
    } else if msg.contains("not in request context") {
        "context"
    } else {
        "io"
    }
}

/// Log a server-fn outcome once at the boundary (no duplicate helper logs).
#[cfg(feature = "ssr")]
pub(super) fn trace_server_result<T>(
    operation: &'static str,
    result: &Result<T, ServerFnError>,
    task_name: Option<&str>,
    job_id: Option<&str>,
    run_id: Option<&str>,
) {
    match result {
        Ok(_) => {
            tracing::debug!(
                operation,
                outcome = "ok",
                task_name,
                job_id,
                run_id,
                "boson-app server fn ok"
            );
        }
        Err(err) => {
            let msg = err.to_string();
            let class = error_class(&msg);
            tracing::warn!(
                operation,
                outcome = "err",
                error_class = class,
                task_name,
                job_id,
                run_id,
                error = %msg,
                "boson-app server fn failed"
            );
        }
    }
}
