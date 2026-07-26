use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use leptos::prelude::*;
use orbital::primitives::PageFetcher;
use orbital_data::DataRecord;
use orbital_paging::{Page, PageRequest};

use crate::server::{list_jobs_datatable_page, JOBS_PAGE_SIZE};

use super::mapper::job_to_record;

pub const QUEUE_TABLE_PAGE_SIZE: u32 = JOBS_PAGE_SIZE;

pub fn build_queue_fetcher() -> PageFetcher {
    Arc::new(|request: PageRequest| {
        Box::pin(async move {
            let page = list_jobs_datatable_page(request).await?;
            Ok(Page {
                items: page.items.into_iter().map(job_to_record).collect(),
                has_more: page.has_more,
                total_count: page.total_count,
                next_request_offset: page.next_request_offset,
            })
        }) as Pin<Box<dyn Future<Output = Result<Page<DataRecord>, ServerFnError>> + Send>>
    })
}
