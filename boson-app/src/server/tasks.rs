//! Task list, detail, config, and paginated task endpoints.

use leptos::prelude::*;
use orbital_paging::{Page, PageRequest};

#[cfg(feature = "ssr")]
use super::helpers::{
    aggregate_task_stats, build_task_summary, ensure_verified_user, retry_policy_to_dto,
    task_summary_from_parts,
};
use super::types::{TaskConfigDto, TaskSummary, UpdateTaskConfigRequest};

#[cfg(feature = "ssr")]
use boson_core::TaskConfig;

/// Get all tasks with effective config and stats.
#[uf_product_macros::server]
pub async fn get_tasks() -> Result<Vec<TaskSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    let backend = ctx.boson()?;
    let registry = backend.registry();
    let jobs = backend.list_jobs(None, 0, usize::MAX).await;
    let runs = backend.list_runs(None, 0, usize::MAX).await;
    let stats_by_task = aggregate_task_stats(&jobs, &runs);

    let mut tasks = Vec::new();
    for desc in registry.iter() {
        let name = desc.name.to_string();
        let config = backend
            .get_task_config(&name)
            .await
            .unwrap_or_else(|_| TaskConfig::default_for(&name));
        let stats = stats_by_task.get(&name).copied().unwrap_or_default();
        tasks.push(task_summary_from_parts(desc, &config, stats));
    }
    tasks.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(tasks)
}

/// Get a single task by name (O(1) registry lookup + task-scoped stats).
#[uf_product_macros::server]
pub async fn get_task(task_name: String) -> Result<Option<TaskSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    let backend = ctx.boson()?;
    let desc = match backend.registry().get(&task_name) {
        Some(d) => d,
        None => return Ok(None),
    };
    Ok(Some(build_task_summary(backend, desc).await?))
}

/// Get task config.
#[uf_product_macros::server]
pub async fn get_task_config(task_name: String) -> Result<TaskConfigDto, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    let backend = ctx.boson()?;
    let config = backend
        .get_task_config(&task_name)
        .await
        .map_err(|e| ServerFnError::new(format!("Task config not found: {}", e)))?;
    Ok(TaskConfigDto {
        task_name: config.task_name,
        priority: config.priority,
        pool: config.pool,
        retry_policy: retry_policy_to_dto(&config.retry_policy),
        updated_at: config.updated_at.to_rfc3339(),
    })
}

/// Update task config.
#[uf_product_macros::server]
pub async fn update_task_config(
    task_name: String,
    req: UpdateTaskConfigRequest,
) -> Result<TaskConfigDto, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    ensure_verified_user(&ctx)?;
    let backend = ctx.boson()?;
    let mut config = backend
        .get_task_config(&task_name)
        .await
        .map_err(|e| ServerFnError::new(format!("Task config not found: {}", e)))?;

    if let Some(p) = req.priority {
        config.priority = p;
    }
    if let Some(p) = req.pool {
        config.pool = p;
    }
    if let Some(r) = req.retry_policy {
        config.retry_policy = boson_core::RetryPolicy {
            max_attempts: r.max_attempts,
            base_delay_ms: r.base_delay_ms,
            backoff_multiplier: r.backoff_multiplier,
            max_delay_ms: r.max_delay_ms,
        };
    }
    config.updated_at = chrono::Utc::now();
    backend
        .upsert_task_config(config.clone())
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to update config: {}", e)))?;

    Ok(TaskConfigDto {
        task_name: config.task_name,
        priority: config.priority,
        pool: config.pool,
        retry_policy: retry_policy_to_dto(&config.retry_policy),
        updated_at: config.updated_at.to_rfc3339(),
    })
}

/// Paginated tasks endpoint.
///
/// Returns a [`Page<TaskSummary>`] using the standard `orbital-paging`
/// over-fetch pattern. Tasks come from the in-memory registry (typically
/// small) so we fetch all, sort, filter, then slice.
#[uf_product_macros::server]
pub async fn get_tasks_page(
    offset: u32,
    limit: u32,
    query: Option<String>,
) -> Result<Page<TaskSummary>, ServerFnError> {
    let mut tasks = get_tasks().await?;
    tasks.sort_by(|a, b| a.name.cmp(&b.name));

    if let Some(ref q) = query {
        let q_lower = q.trim().to_lowercase();
        if !q_lower.is_empty() {
            tasks.retain(|t| {
                t.name.to_lowercase().contains(&q_lower)
                    || t.signature_json.to_lowercase().contains(&q_lower)
                    || t.effective_pool.to_lowercase().contains(&q_lower)
            });
        }
    }

    let total_count: Option<u64> = if offset == 0 {
        Some(tasks.len() as u64)
    } else {
        None
    };

    let sliced: Vec<TaskSummary> = tasks
        .into_iter()
        .skip(offset as usize)
        .take((limit + 1) as usize)
        .collect();

    Ok(Page::from_oversized(sliced, limit, total_count))
}

/// Paginated tasks for DataTable toolbar (quick search via PageRequest).
#[uf_product_macros::server]
pub async fn get_tasks_datatable_page(
    request: PageRequest,
) -> Result<Page<TaskSummary>, ServerFnError> {
    get_tasks_page(
        request.offset,
        request.limit,
        super::page_query::quick_search_text(&request),
    )
    .await
}
