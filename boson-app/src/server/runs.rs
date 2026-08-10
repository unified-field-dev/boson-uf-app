//! Run history server functions.

use leptos::prelude::*;
use orbital_paging::{Page, PageRequest};

#[cfg(feature = "ssr")]
use super::helpers::{require_session, run_to_summary};
use super::page_query;
use super::types::{clamp_page_list_limit, RunSummary, BOSON_LIST_FETCH_CAP};

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
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
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

/// Paginated runs for DataTable with in-memory filter/search (bounded fetch).
#[uf_product_macros::server]
pub async fn list_runs_datatable_page(
    /// `DataTable` paging/filter/search/sort request from the client.
    request: PageRequest,
    /// Optional job id to scope results to a single job's runs.
    scope_job_id: Option<String>,
) -> Result<Page<RunSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let limit = clamp_page_list_limit(request.limit);
    let job_filter = page_query::resolve_job_filter(scope_job_id, &request);
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

/// Get a single run by id.
#[uf_product_macros::server]
pub async fn get_run(
    /// Unique identifier of the run to look up.
    run_id: String,
) -> Result<Option<RunSummary>, ServerFnError> {
    boson_backend::validate_run_id(&run_id).map_err(ServerFnError::new)?;
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let backend = super::helpers::boson_backend()?;
    let backend = backend.as_ref();
    Ok(backend.get_run(&run_id).await.map(run_to_summary))
}
