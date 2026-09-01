//! Run history server functions.

use leptos::prelude::*;
use orbital_paging::{Page, PageRequest};

#[cfg(feature = "ssr")]
use super::helpers::{require_session, run_to_summary, trace_server_result};
#[cfg(feature = "ssr")]
use super::page_query;
use super::types::RunSummary;
#[cfg(feature = "ssr")]
use super::types::{clamp_page_list_limit, BOSON_LIST_FETCH_CAP};

/// Paginated runs endpoint.
#[uf_product_macros::server]
pub async fn list_runs_page(
    /// Zero-based index of the first run to return.
    offset: u32,
    /// Maximum number of runs to return.
    limit: u32,
    /// Optional job id to restrict results to runs of a single job.
    job_id_filter: Option<String>,
) -> Result<Page<RunSummary>, ServerFnError> {
    let result = async {
        let ctx = higgs::Higgs::from_request().await?;
        require_session(&ctx)?;
        if let Some(ref job_id) = job_id_filter {
            boson_backend::validate_job_id(job_id)
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        }
        let limit = clamp_page_list_limit(limit);
        let backend = super::helpers::boson_backend()?;
        let backend = backend.as_ref();

        let runs = backend
            .list_runs(
                job_id_filter.as_deref(),
                offset as usize,
                (limit + 1) as usize,
            )
            .await;

        let dtos: Vec<RunSummary> = runs.into_iter().map(run_to_summary).collect();

        let total_count: Option<u64> = if offset == 0 {
            Some(backend.count_runs(job_id_filter.as_deref()).await)
        } else {
            None
        };

        Ok(Page::from_oversized(dtos, limit, total_count))
    }
    .await;
    #[cfg(feature = "ssr")]
    trace_server_result("list_runs_page", &result, None, None, None);
    result
}

/// Paginated runs for DataTable with in-memory filter/search (bounded fetch).
#[uf_product_macros::server]
pub async fn list_runs_datatable_page(
    /// `DataTable` paging/filter/search/sort request from the client.
    request: PageRequest,
    /// Optional job id to scope results to a single job's runs.
    scope_job_id: Option<String>,
) -> Result<Page<RunSummary>, ServerFnError> {
    let result = async {
        let ctx = higgs::Higgs::from_request().await?;
        require_session(&ctx)?;
        let limit = clamp_page_list_limit(request.limit);
        let job_filter = page_query::resolve_job_filter(scope_job_id, &request);
        if let Some(ref job_id) = job_filter {
            boson_backend::validate_job_id(job_id)
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        }
        let backend = super::helpers::boson_backend()?;
        let backend = backend.as_ref();

        let runs = backend
            .list_runs(job_filter.as_deref(), 0, BOSON_LIST_FETCH_CAP)
            .await;

        let mut dtos: Vec<RunSummary> = runs.into_iter().map(run_to_summary).collect();

        page_query::apply_runs_datatable_query(&mut dtos, &request);

        let total_count = if request.is_first_page() {
            Some(dtos.len() as u64)
        } else {
            None
        };

        let sliced: Vec<RunSummary> = dtos
            .into_iter()
            .skip(request.offset as usize)
            .take((limit + 1) as usize)
            .collect();

        Ok(Page::from_oversized(sliced, limit, total_count))
    }
    .await;
    #[cfg(feature = "ssr")]
    trace_server_result("list_runs_datatable_page", &result, None, None, None);
    result
}

/// Get a single run by id.
#[uf_product_macros::server]
pub async fn get_run(
    /// Unique identifier of the run to look up.
    run_id: String,
) -> Result<Option<RunSummary>, ServerFnError> {
    let id_for_trace = run_id.clone();
    let result = async {
        boson_backend::validate_run_id(&run_id).map_err(|e| ServerFnError::new(e.to_string()))?;
        let ctx = higgs::Higgs::from_request().await?;
        require_session(&ctx)?;
        let backend = super::helpers::boson_backend()?;
        let backend = backend.as_ref();
        Ok(backend.get_run(&run_id).await.map(run_to_summary))
    }
    .await;
    #[cfg(feature = "ssr")]
    trace_server_result("get_run", &result, None, None, Some(&id_for_trace));
    result
}
