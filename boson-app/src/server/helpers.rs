//! Auth helpers and task summary builders for Boson server functions.
//!
//! Pure DTO mappers and aggregates live in [`boson_backend`].

#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;

#[cfg(feature = "ssr")]
pub(super) use boson_backend::{
    aggregate_task_stats, apply_task_config_update, job_to_summary, parse_job_status_filter,
    retry_policy_to_dto, run_to_summary, task_config_to_dto, task_summary_from_parts, TaskStatsAgg,
};

#[cfg(feature = "ssr")]
pub(super) fn boson_backend(
) -> Result<std::sync::Arc<dyn boson_coordinator::BosonCoordinatorBackend>, ServerFnError> {
    leptos::context::use_context::<std::sync::Arc<dyn boson_coordinator::BosonCoordinatorBackend>>()
        .ok_or_else(|| ServerFnError::new("Boson backend not in request context"))
}

#[cfg(feature = "ssr")]
pub(super) fn ensure_verified_user(ctx: &higgs::Higgs) -> Result<(), ServerFnError> {
    if ctx.session_user_id().is_some() {
        Ok(())
    } else {
        Err(ServerFnError::new(
            "Authentication is required for this action",
        ))
    }
}

/// Build a single task summary via registry descriptor and backend stats.
#[cfg(feature = "ssr")]
pub(super) async fn build_task_summary(
    backend: &dyn boson_coordinator::BosonCoordinatorBackend,
    desc: &boson_runtime::TaskDescriptor,
) -> Result<super::types::TaskSummary, ServerFnError> {
    use boson_core::{JobStatus, TaskConfig};

    let name = desc.name.to_string();
    let config = backend
        .get_task_config(&name)
        .await
        .unwrap_or_else(|_| TaskConfig::default_for(&name));

    let jobs_queued = backend
        .count_jobs_for_task(&name, Some(JobStatus::Queued))
        .await as u32;
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
