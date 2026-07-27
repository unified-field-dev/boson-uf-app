//! Job queue server functions.

use leptos::prelude::*;
use orbital_paging::{Page, PageRequest};

#[cfg(feature = "ssr")]
use super::helpers::{job_to_summary, parse_job_status_filter, require_session};
use super::page_query;
use super::types::{JobSummary, BOSON_LIST_FETCH_CAP, clamp_page_list_limit};

/// Cancel a job.
#[uf_product_macros::server(permission = "BosonAdmin")]
pub async fn cancel_job(
    /// Unique identifier of the job to cancel.
    job_id: String,
) -> Result<(), ServerFnError> {
    boson_backend::validate_job_id(&job_id).map_err(ServerFnError::new)?;
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let backend = super::helpers::boson_backend()?;
    let backend = backend.as_ref();
    backend
        .cancel_job(&job_id)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to cancel job: {}", e)))
}

/// Paginated jobs endpoint.
#[uf_product_macros::server]
pub async fn list_jobs_page(
    /// Zero-based index of the first job to return.
    offset: u32,
    /// Maximum number of jobs to return.
    limit: u32,
    /// Optional job status name to filter by (e.g. "queued", "running").
    status_filter: Option<String>,
) -> Result<Page<JobSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let limit = clamp_page_list_limit(limit);
    let backend = super::helpers::boson_backend()?;
    let backend = backend.as_ref();
    let status = status_filter.as_deref().and_then(parse_job_status_filter);

    let jobs = backend
        .list_jobs(status, offset as usize, (limit + 1) as usize)
        .await;

    let dtos: Vec<JobSummary> = jobs.into_iter().map(job_to_summary).collect();

    let total_count: Option<u64> = if offset == 0 {
        Some(backend.count_jobs(status).await)
    } else {
        None
    };

    Ok(Page::from_oversized(dtos, limit, total_count))
}

/// Paginated jobs for DataTable with status + quick search filters.
#[uf_product_macros::server]
pub async fn list_jobs_datatable_page(
    /// `DataTable` paging/filter/search/sort request from the client.
    request: PageRequest,
) -> Result<Page<JobSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let limit = clamp_page_list_limit(request.limit);
    let status_filter = page_query::extract_status_filter(&request);
    let needs_memory_filter = page_query::quick_search_text(&request).is_some()
        || request
            .filter
            .as_ref()
            .is_some_and(|f| f.items.iter().any(|r| r.field != "status"));

    if needs_memory_filter {
        let backend = super::helpers::boson_backend()?;
        let backend = backend.as_ref();
        let status = status_filter.as_deref().and_then(parse_job_status_filter);

        let jobs = backend.list_jobs(status, 0, BOSON_LIST_FETCH_CAP).await;
        let mut dtos: Vec<JobSummary> = jobs.into_iter().map(job_to_summary).collect();

        page_query::apply_jobs_datatable_query(&mut dtos, &request);

        let total_count = if request.is_first_page() {
            Some(dtos.len() as u64)
        } else {
            None
        };

        let sliced: Vec<JobSummary> = dtos
            .into_iter()
            .skip(request.offset as usize)
            .take((limit + 1) as usize)
            .collect();

        Ok(Page::from_oversized(sliced, limit, total_count))
    } else {
        list_jobs_page(request.offset, limit, status_filter).await
    }
}
