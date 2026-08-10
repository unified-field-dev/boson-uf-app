//! Task list, detail, config, and paginated task endpoints.

use leptos::prelude::*;
use orbital_paging::{Page, PageRequest};

#[cfg(feature = "ssr")]
use super::helpers::{
    apply_task_config_update, build_task_summary, require_email_verified, require_session,
    task_config_to_dto,
};
use super::types::{clamp_page_list_limit, TaskConfigDto, TaskSummary, UpdateTaskConfigRequest};

/// Get all tasks with effective config and stats.
#[uf_product_macros::server]
pub async fn get_tasks() -> Result<Vec<TaskSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let backend = super::helpers::boson_backend()?;
    let backend = backend.as_ref();
    let registry = backend.registry();

    let mut tasks = Vec::new();
    for desc in registry.iter() {
        tasks.push(build_task_summary(backend, desc).await?);
    }
    boson_backend::sort_tasks_by_name(&mut tasks);
    Ok(tasks)
}

/// Get a single task by name (O(1) registry lookup + task-scoped stats).
#[uf_product_macros::server]
pub async fn get_task(
    /// Registry name of the task to look up.
    task_name: String,
) -> Result<Option<TaskSummary>, ServerFnError> {
    boson_backend::validate_task_name(&task_name).map_err(ServerFnError::new)?;
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let backend = super::helpers::boson_backend()?;
    let backend = backend.as_ref();
    let desc = match backend.registry().get(&task_name) {
        Some(d) => d,
        None => return Ok(None),
    };
    Ok(Some(build_task_summary(backend, desc).await?))
}

/// Get task config.
#[uf_product_macros::server(permission = "BosonAdmin")]
pub async fn get_task_config(
    /// Registry name of the task whose config should be fetched.
    task_name: String,
) -> Result<TaskConfigDto, ServerFnError> {
    boson_backend::validate_task_name(&task_name).map_err(ServerFnError::new)?;
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    require_email_verified().await?;
    let backend = super::helpers::boson_backend()?;
    let backend = backend.as_ref();
    let config = backend
        .get_task_config(&task_name)
        .await
        .map_err(|e| ServerFnError::new(format!("Task config not found: {}", e)))?;
    Ok(task_config_to_dto(&config))
}

/// Update task config.
#[uf_product_macros::server(permission = "BosonAdmin")]
pub async fn update_task_config(
    /// Registry name of the task whose config should be updated.
    task_name: String,
    /// Partial update request with the fields to change.
    req: UpdateTaskConfigRequest,
) -> Result<TaskConfigDto, ServerFnError> {
    boson_backend::validate_task_name(&task_name).map_err(ServerFnError::new)?;
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    require_email_verified().await?;
    let backend = super::helpers::boson_backend()?;
    let backend = backend.as_ref();
    let mut config = backend
        .get_task_config(&task_name)
        .await
        .map_err(|e| ServerFnError::new(format!("Task config not found: {}", e)))?;

    apply_task_config_update(&mut config, &req, chrono::Utc::now());
    backend
        .upsert_task_config(config.clone())
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to update config: {}", e)))?;

    Ok(task_config_to_dto(&config))
}

/// Paginated tasks endpoint.
///
/// Returns a [`Page<TaskSummary>`] using the standard `orbital-paging`
/// over-fetch pattern. Tasks come from the in-memory registry (typically
/// small) so we fetch all, sort, filter, then slice.
#[uf_product_macros::server]
pub async fn get_tasks_page(
    /// Zero-based index of the first task to return.
    offset: u32,
    /// Maximum number of tasks to return.
    limit: u32,
    /// Optional case-insensitive search string matched against name, signature, and pool.
    query: Option<String>,
) -> Result<Page<TaskSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let limit = clamp_page_list_limit(limit);
    let mut tasks = get_tasks().await?;
    boson_backend::sort_tasks_by_name(&mut tasks);
    boson_backend::filter_tasks_by_query(&mut tasks, query.as_deref());

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
    /// `DataTable` paging/filter/search/sort request from the client.
    request: PageRequest,
) -> Result<Page<TaskSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let limit = clamp_page_list_limit(request.limit);
    get_tasks_page(
        request.offset,
        limit,
        super::page_query::quick_search_text(&request),
    )
    .await
}
