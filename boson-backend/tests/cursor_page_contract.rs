//! Cursor paging contracts for `list_jobs_datatable_page` / `list_runs_datatable_page`.
//!
//! `serve_jobs` / `serve_runs` repeat the server functions' cursor-path reads over an in-memory
//! collection, with `boson_core::page_jobs_after` / `page_runs_after` standing in for the
//! coordinator's keyset methods.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashSet;

use boson_backend::{
    extract_status_filter, jobs_cursor_eligible, jobs_list_start, jobs_page_from_rows,
    parse_job_status_filter, resolve_job_filter, runs_cursor_eligible, runs_cursor_scope,
    runs_list_start, runs_page_from_rows, JobSummary, ListStart, RunSummary, JOBS_CURSOR_SCOPE,
};
use boson_core::{page_jobs_after, page_runs_after, Job, JobStatus, Run};
use chrono::{DateTime, Duration, TimeZone, Utc};
use orbital_data::DataValue;
use orbital_paging::{
    FilterLogicWire, FilterQuery, FilterRuleParam, Page, PageCursor, PageRequest,
};

const PAGE: u32 = 25;

fn base() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap()
}

const fn status_of(i: usize) -> JobStatus {
    [JobStatus::Queued, JobStatus::Success, JobStatus::Failed][i % 3]
}

/// Groups of four share a sub-second `created_at`, so the id tie-break and nanosecond keys
/// both matter.
fn jobs(n: usize) -> Vec<Job> {
    (0..n)
        .rev()
        .map(|i| {
            let mut j = Job::new(
                "t",
                serde_json::json!({}),
                serde_json::json!({}),
                0,
                "global",
                0,
                None,
            );
            j.job_id = format!("job-{i:06}");
            j.created_at = base() + Duration::nanoseconds(i64::try_from(i / 4).unwrap() * 1_337);
            j.status = status_of(i);
            j
        })
        .collect()
}

fn runs(n: usize) -> Vec<Run> {
    (0..n)
        .map(|i| {
            let job = if i % 2 == 0 { "job-a" } else { "job-b" };
            let mut r = Run::new(job, "t", 1);
            r.run_id = format!("run-{i:06}");
            r.started_at = base() + Duration::nanoseconds(i64::try_from(i / 4).unwrap() * 1_337);
            r
        })
        .collect()
}

fn rule(field: &str, value: &str) -> FilterRuleParam {
    FilterRuleParam {
        field: field.into(),
        operator: "equals".into(),
        value: DataValue::Text(value.into()),
    }
}

fn request(offset: u32, filter: Vec<FilterRuleParam>, cursor: Option<String>) -> PageRequest {
    let filter = (!filter.is_empty()).then_some(FilterQuery {
        items: filter,
        logic: FilterLogicWire::And,
    });
    PageRequest::with_query(offset, PAGE, None, filter, None).with_cursor(cursor)
}

/// The cursor path of `list_jobs_datatable_page` over `all`.
fn serve_jobs(
    all: &[Job],
    request: &PageRequest,
) -> (ListStart<boson_core::JobKey>, Page<JobSummary>) {
    assert!(jobs_cursor_eligible(request));
    let status = extract_status_filter(request)
        .as_deref()
        .and_then(parse_job_status_filter);
    let start = jobs_list_start(request);
    let want = (PAGE + 1) as usize;
    let rows: Vec<Job> = match &start {
        ListStart::After(key) => page_jobs_after(all.iter(), status, key.as_ref(), want)
            .into_iter()
            .cloned()
            .collect(),
        ListStart::Offset(offset) => page_jobs_after(all.iter(), status, None, usize::MAX)
            .into_iter()
            .skip(*offset)
            .take(want)
            .cloned()
            .collect(),
    };
    let page = jobs_page_from_rows(rows, request, PAGE, &start, None);
    (start, page)
}

/// The cursor path of `list_runs_datatable_page` over `all`.
fn serve_runs(
    all: &[Run],
    request: &PageRequest,
    scope: Option<&str>,
) -> (ListStart<boson_core::RunKey>, Page<RunSummary>) {
    assert!(runs_cursor_eligible(request, scope));
    let job = resolve_job_filter(scope.map(str::to_string), request);
    let start = runs_list_start(request, scope);
    let want = (PAGE + 1) as usize;
    let rows: Vec<Run> = match &start {
        ListStart::After(key) => page_runs_after(all.iter(), job.as_deref(), key.as_ref(), want)
            .into_iter()
            .cloned()
            .collect(),
        ListStart::Offset(offset) => page_runs_after(all.iter(), job.as_deref(), None, usize::MAX)
            .into_iter()
            .skip(*offset)
            .take(want)
            .cloned()
            .collect(),
    };
    let page = runs_page_from_rows(rows, request, scope, PAGE, &start, None);
    (start, page)
}

/// Follow `next_cursor` from the first page to the end, as the `DataTable` does on Next.
fn page_all_jobs(all: &[Job], filter: &[FilterRuleParam]) -> Vec<String> {
    let mut seen = Vec::new();
    let mut offset = 0;
    let mut cursor = None;
    loop {
        let req = request(offset, filter.to_vec(), cursor.take());
        let (start, page) = serve_jobs(all, &req);
        assert!(
            matches!(start, ListStart::After(_)),
            "page at {offset} fell back to offset"
        );
        seen.extend(page.items.iter().map(|j| j.job_id.clone()));
        assert_eq!(page.has_more, page.next_cursor.is_some());
        if !page.has_more {
            break;
        }
        offset += PAGE;
        cursor = page.next_cursor;
    }
    seen
}

#[test]
fn jobs_cursor_pages_2500_once_in_order_happy_path() {
    let all = jobs(2_500);
    let seen = page_all_jobs(&all, &[]);
    let expected: Vec<String> = (0..2_500).map(|i| format!("job-{i:06}")).collect();
    assert_eq!(seen, expected);
}

#[test]
fn jobs_cursor_pages_with_status_filter_happy_path() {
    let all = jobs(2_500);
    let seen = page_all_jobs(&all, &[rule("status", "success")]);
    let expected: Vec<String> = (0..2_500)
        .filter(|i| status_of(*i) == JobStatus::Success)
        .map(|i| format!("job-{i:06}"))
        .collect();
    assert_eq!(seen.len(), expected.len());
    assert_eq!(seen.iter().collect::<HashSet<_>>().len(), seen.len());
    assert_eq!(seen, expected);
}

#[test]
fn jobs_cursor_skips_nothing_when_seen_rows_leave_the_filter_edge() {
    // Offset paging skips rows here: once seen queued jobs start running, every later offset
    // lands further into the queue than intended.
    let mut all = jobs(600);
    let filter = vec![rule("status", "queued")];
    let (_, first) = serve_jobs(&all, &request(0, filter.clone(), None));
    for job in &mut all {
        if first.items.iter().any(|j| j.job_id == job.job_id) {
            job.status = JobStatus::Running;
        }
    }
    let mut seen: Vec<String> = first.items.iter().map(|j| j.job_id.clone()).collect();
    let mut cursor = first.next_cursor;
    let mut offset = PAGE;
    while let Some(token) = cursor {
        let (start, page) = serve_jobs(&all, &request(offset, filter.clone(), Some(token)));
        assert!(matches!(start, ListStart::After(Some(_))));
        seen.extend(page.items.iter().map(|j| j.job_id.clone()));
        cursor = page.next_cursor;
        offset += PAGE;
    }
    let expected: Vec<String> = (0..600)
        .filter(|i| status_of(*i) == JobStatus::Queued)
        .map(|i| format!("job-{i:06}"))
        .collect();
    assert_eq!(seen, expected);
}

#[test]
fn jobs_cursor_from_other_filter_falls_back_to_offset_sad_path() {
    let all = jobs(200);
    let (_, queued) = serve_jobs(&all, &request(0, vec![rule("status", "queued")], None));
    let token = queued.next_cursor.expect("queued has more");

    let req = request(PAGE, vec![rule("status", "success")], Some(token));
    let (start, page) = serve_jobs(&all, &req);
    assert_eq!(start, ListStart::Offset(PAGE as usize));
    assert!(page.next_cursor.is_none(), "offset pages return no cursor");
    let expected: Vec<String> = (0..200)
        .filter(|i| status_of(*i) == JobStatus::Success)
        .skip(PAGE as usize)
        .take(PAGE as usize)
        .map(|i| format!("job-{i:06}"))
        .collect();
    let got: Vec<String> = page.items.into_iter().map(|j| j.job_id).collect();
    assert_eq!(got, expected);
}

#[test]
fn jobs_garbage_cursor_falls_back_to_offset_sad_path() {
    let fp = request(0, vec![], None).query_fingerprint(JOBS_CURSOR_SCOPE);
    for token in [
        "not-a-cursor".to_string(),
        PageCursor::with_key(0, "no-separator", fp.clone()).encode(),
        PageCursor::with_key(0, "yesterday|job-000001", fp.clone()).encode(),
        PageCursor::with_key(0, "2026-01-01T00:00:00Z|../etc", fp.clone()).encode(),
        PageCursor::new(25, fp).encode(),
    ] {
        let req = request(50, vec![], Some(token.clone()));
        assert_eq!(
            jobs_list_start(&req),
            ListStart::Offset(50),
            "token {token}"
        );
    }
    // Without a usable cursor the first page still starts the keyset scan.
    let req = request(0, vec![], Some("not-a-cursor".into()));
    assert_eq!(jobs_list_start(&req), ListStart::After(None));
}

#[test]
fn jobs_quick_search_or_other_filter_stays_off_cursor_path_edge() {
    let mut req = request(0, vec![], None);
    req.quick_search = Some("alpha".into());
    assert!(!jobs_cursor_eligible(&req));
    assert!(!jobs_cursor_eligible(&request(
        0,
        vec![rule("pool", "global")],
        None
    )));
    assert!(jobs_cursor_eligible(&request(
        0,
        vec![rule("status", "queued")],
        None
    )));
}

#[test]
fn runs_cursor_pages_job_scope_once_in_order_happy_path() {
    let all = runs(1_200);
    let scope = Some("job-a");
    let mut seen = Vec::new();
    let mut offset = 0;
    let mut cursor = None;
    loop {
        let req = request(offset, vec![], cursor.take());
        let (start, page) = serve_runs(&all, &req, scope);
        assert!(matches!(start, ListStart::After(_)));
        seen.extend(page.items.iter().map(|r| r.run_id.clone()));
        if !page.has_more {
            break;
        }
        offset += PAGE;
        cursor = page.next_cursor;
    }
    let expected: Vec<String> = (0..1_200_usize)
        .rev()
        .filter(|i| i % 2 == 0)
        .map(|i| format!("run-{i:06}"))
        .collect();
    assert_eq!(seen, expected);
}

#[test]
fn runs_cursor_from_other_job_scope_falls_back_to_offset_sad_path() {
    let all = runs(200);
    let (_, page) = serve_runs(&all, &request(0, vec![], None), Some("job-a"));
    let token = page.next_cursor.expect("job-a has more");
    let req = request(PAGE, vec![], Some(token));
    assert_eq!(
        runs_list_start(&req, Some("job-b")),
        ListStart::Offset(PAGE as usize)
    );
    assert_ne!(
        runs_cursor_scope(Some("job-a")),
        runs_cursor_scope(Some("job-b"))
    );
}

#[test]
fn runs_cursor_eligibility_edge() {
    assert!(runs_cursor_eligible(
        &request(0, vec![], None),
        Some("job-a")
    ));
    assert!(runs_cursor_eligible(
        &request(0, vec![rule("job_id", "job-a")], None),
        None
    ));
    // A job_id rule on top of a URL scope, or any other rule, stays in memory.
    assert!(!runs_cursor_eligible(
        &request(0, vec![rule("job_id", "job-b")], None),
        Some("job-a")
    ));
    assert!(!runs_cursor_eligible(
        &request(0, vec![rule("status", "failed")], None),
        None
    ));
    let mut req = request(0, vec![], None);
    req.quick_search = Some("run-1".into());
    assert!(!runs_cursor_eligible(&req, None));
}
