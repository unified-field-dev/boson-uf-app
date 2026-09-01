//! Task list, detail, config, and paginated task endpoints.

use leptos::prelude::*;
use orbital_paging::{Page, PageRequest};

#[cfg(feature = "ssr")]
use super::helpers::{
    apply_task_config_update, build_task_summary, load_all_task_summaries, require_email_verified,
    require_session, task_config_to_dto, trace_server_result,
};
use super::types::{clamp_page_list_limit, TaskConfigDto, TaskSummary, UpdateTaskConfigRequest};

/// Get all tasks with effective config and stats.
#[uf_product_macros::server]
pub async fn get_tasks() -> Result<Vec<TaskSummary>, ServerFnError> {
    let result = async {
        let ctx = higgs::Higgs::from_request().await?;
        require_session(&ctx)?;
        let backend = super::helpers::boson_backend()?;
        load_all_task_summaries(backend.as_ref()).await
    }
    .await;
    #[cfg(feature = "ssr")]
    trace_server_result("get_tasks", &result, None, None, None);
    result
}

/// Get a single task by name (O(1) registry lookup + task-scoped stats).
#[uf_product_macros::server]
pub async fn get_task(
    /// Registry name of the task to look up.
    task_name: String,
) -> Result<Option<TaskSummary>, ServerFnError> {
    let name_for_trace = task_name.clone();
    let result = async {
        boson_backend::validate_task_name(&task_name)
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        let ctx = higgs::Higgs::from_request().await?;
        require_session(&ctx)?;
        let backend = super::helpers::boson_backend()?;
        let backend = backend.as_ref();
        let Some(desc) = backend.registry().get(&task_name) else {
            return Ok(None);
        };
        Ok(Some(build_task_summary(backend, desc).await?))
    }
    .await;
    #[cfg(feature = "ssr")]
    trace_server_result("get_task", &result, Some(&name_for_trace), None, None);
    result
}

/// Get task config.
#[uf_product_macros::server(permission = "BosonAdmin")]
pub async fn get_task_config(
    /// Registry name of the task whose config should be fetched.
    task_name: String,
) -> Result<TaskConfigDto, ServerFnError> {
    let name_for_trace = task_name.clone();
    let result = async {
        boson_backend::validate_task_name(&task_name)
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        let ctx = higgs::Higgs::from_request().await?;
        require_session(&ctx)?;
        require_email_verified().await?;
        let backend = super::helpers::boson_backend()?;
        let backend = backend.as_ref();
        let config = backend
            .get_task_config(&task_name)
            .await
            .map_err(|e| ServerFnError::new(format!("Task config not found: {e}")))?;
        Ok(task_config_to_dto(&config))
    }
    .await;
    #[cfg(feature = "ssr")]
    trace_server_result(
        "get_task_config",
        &result,
        Some(&name_for_trace),
        None,
        None,
    );
    result
}

/// Update task config.
#[uf_product_macros::server(permission = "BosonAdmin")]
pub async fn update_task_config(
    /// Registry name of the task whose config should be updated.
    task_name: String,
    /// Partial update request with the fields to change.
    req: UpdateTaskConfigRequest,
) -> Result<TaskConfigDto, ServerFnError> {
    let name_for_trace = task_name.clone();
    let result = async {
        boson_backend::validate_task_name(&task_name)
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        let ctx = higgs::Higgs::from_request().await?;
        require_session(&ctx)?;
        require_email_verified().await?;
        boson_backend::validate_task_config_update(&req)
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        let backend = super::helpers::boson_backend()?;
        let backend = backend.as_ref();
        let mut config = backend
            .get_task_config(&task_name)
            .await
            .map_err(|e| ServerFnError::new(format!("Task config not found: {e}")))?;

        apply_task_config_update(&mut config, &req, chrono::Utc::now());
        backend
            .upsert_task_config(config.clone())
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to update config: {e}")))?;

        Ok(task_config_to_dto(&config))
    }
    .await;
    #[cfg(feature = "ssr")]
    trace_server_result(
        "update_task_config",
        &result,
        Some(&name_for_trace),
        None,
        None,
    );
    result
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
    let result = async {
        let ctx = higgs::Higgs::from_request().await?;
        require_session(&ctx)?;
        let limit = clamp_page_list_limit(limit);
        let backend = super::helpers::boson_backend()?;
        let mut tasks = load_all_task_summaries(backend.as_ref()).await?;
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
    .await;
    #[cfg(feature = "ssr")]
    trace_server_result("get_tasks_page", &result, None, None, None);
    result
}

/// Paginated tasks for DataTable toolbar (quick search via PageRequest).
#[uf_product_macros::server]
pub async fn get_tasks_datatable_page(
    /// `DataTable` paging/filter/search/sort request from the client.
    request: PageRequest,
) -> Result<Page<TaskSummary>, ServerFnError> {
    let result = async {
        let ctx = higgs::Higgs::from_request().await?;
        require_session(&ctx)?;
        let limit = clamp_page_list_limit(request.limit);
        let backend = super::helpers::boson_backend()?;
        let mut tasks = load_all_task_summaries(backend.as_ref()).await?;
        boson_backend::filter_tasks_by_query(
            &mut tasks,
            super::page_query::quick_search_text(&request).as_deref(),
        );

        let total_count: Option<u64> = if request.offset == 0 {
            Some(tasks.len() as u64)
        } else {
            None
        };

        let sliced: Vec<TaskSummary> = tasks
            .into_iter()
            .skip(request.offset as usize)
            .take((limit + 1) as usize)
            .collect();

        Ok(Page::from_oversized(sliced, limit, total_count))
    }
    .await;
    #[cfg(feature = "ssr")]
    trace_server_result("get_tasks_datatable_page", &result, None, None, None);
    result
}
