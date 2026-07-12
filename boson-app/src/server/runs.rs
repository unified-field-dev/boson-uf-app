//! Run history server functions.

use leptos::prelude::*;
use orbital_paging::{Page, PageRequest};

#[cfg(feature = "ssr")]
use super::helpers::run_to_summary;
use super::page_query;
use super::types::{RunSummary, BOSON_LIST_FETCH_CAP};

/// Paginated runs endpoint.
#[uf_product_macros::server]
pub async fn list_runs_page(
    offset: u32,
    limit: u32,
    job_id_filter: Option<String>,
) -> Result<Page<RunSummary>, ServerFnError> {
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
    request: PageRequest,
    scope_job_id: Option<String>,
) -> Result<Page<RunSummary>, ServerFnError> {
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
        .take((request.limit + 1) as usize)
        .collect();

    Ok(Page::from_oversized(sliced, request.limit, total_count))
}

/// Get a single run by id.
#[uf_product_macros::server]
pub async fn get_run(run_id: String) -> Result<Option<RunSummary>, ServerFnError> {
    let backend = super::helpers::boson_backend()?;
    let backend = backend.as_ref();
    Ok(backend.get_run(&run_id).await.map(run_to_summary))
}
