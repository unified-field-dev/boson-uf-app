use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use leptos::prelude::*;
use orbital::primitives::PageFetcher;
use orbital_data::DataRecord;
use orbital_paging::{Page, PageRequest};

use crate::server::{list_runs_datatable_page, RUNS_PAGE_SIZE};

use super::mapper::run_to_record;
use super::RunsTableScope;

pub const RUNS_TABLE_PAGE_SIZE: u32 = RUNS_PAGE_SIZE;

pub fn build_runs_fetcher(scope: RunsTableScope) -> PageFetcher {
    Arc::new(move |request: PageRequest| {
        let scope_job = match scope.clone() {
            RunsTableScope::All => None,
            RunsTableScope::ForJob(job_id) => Some(job_id),
        };
        Box::pin(async move {
            let page = list_runs_datatable_page(request, scope_job).await?;
            Ok(Page {
                items: page.items.into_iter().map(run_to_record).collect(),
                has_more: page.has_more,
                total_count: page.total_count,
                next_request_offset: page.next_request_offset,
            })
        }) as Pin<Box<dyn Future<Output = Result<Page<DataRecord>, ServerFnError>> + Send>>
    })
}
