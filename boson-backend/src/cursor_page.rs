//! Cursor paging for the jobs and runs `DataTable` server functions.
//!
//! `list_jobs_datatable_page` and `list_runs_datatable_page` take one of two paths:
//!
//! - **Cursor path** — when every filter can be pushed down to the coordinator (jobs: no quick
//!   search and only `status` rules; runs: no quick search and at most one `job_id` equals rule
//!   when the URL has no job scope). The first page and every page that carries a valid cursor
//!   read with the coordinator's keyset methods (`list_jobs_after` in `created_at ASC, job_id
//!   ASC` order, `list_runs_after` in `started_at DESC, run_id DESC` order). Each page with more
//!   rows returns a `next_cursor` holding the last row's key.
//! - **Offset path** — a request past the first page with no cursor, or with a cursor that does
//!   not decode for this query (another filter, page size or job scope, a garbage token, or a key
//!   that does not parse), reads with `list_jobs` / `list_runs` at `request.offset` and returns
//!   no `next_cursor`. Quick search and other filters stay on the bounded in-memory scan in the
//!   server functions and never use a cursor.
//!
//! The cursor key is `"<created_at>|<job_id>"` for jobs and `"<started_at>|<run_id>"` for runs,
//! with the timestamp in RFC 3339 at nanosecond precision. `position` holds the offset the next
//! page would have had; reads ignore it. A cursor only says where to continue: the coordinator
//! read still applies the status and job filters from the request.

use boson_core::{Job, JobKey, Run, RunKey};
use chrono::{DateTime, SecondsFormat, Utc};
use orbital_paging::{Page, PageCursor, PageRequest};

use crate::map::{job_to_summary, run_to_summary};
use crate::page_query::quick_search_text;
use crate::types::{JobSummary, RunSummary};
use crate::validate::{validate_job_id, validate_run_id};

/// Fingerprint scope for jobs table cursors.
pub const JOBS_CURSOR_SCOPE: &str = "boson.jobs";

/// Fingerprint scope for runs table cursors; includes the URL job scope.
#[must_use]
pub fn runs_cursor_scope(scope_job_id: Option<&str>) -> String {
    format!("boson.runs:{}", scope_job_id.unwrap_or_default())
}

/// Where a cursor-path page starts reading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListStart<K> {
    /// Read with the keyset method strictly after this key; `None` reads from the start.
    After(Option<K>),
    /// Read with the offset method from this row.
    Offset(usize),
}

/// True when a jobs `DataTable` request can page by cursor.
///
/// Requires no quick search and only `status` filter rules, which the coordinator applies.
#[must_use]
pub fn jobs_cursor_eligible(request: &PageRequest) -> bool {
    quick_search_text(request).is_none()
        && request
            .filter
            .as_ref()
            .is_none_or(|f| f.items.iter().all(|r| r.field == "status"))
}

/// True when a runs `DataTable` request can page by cursor.
///
/// Requires no quick search, and either no filter rules or (without a URL job scope) a single
/// `job_id` equals rule, which the coordinator applies.
#[must_use]
pub fn runs_cursor_eligible(request: &PageRequest, scope_job_id: Option<&str>) -> bool {
    if quick_search_text(request).is_some() {
        return false;
    }
    let Some(filter) = request.filter.as_ref() else {
        return true;
    };
    match filter.items.as_slice() {
        [] => true,
        [rule] => {
            scope_job_id.is_none_or(str::is_empty)
                && rule.field == "job_id"
                && matches!(rule.operator.as_str(), "equals" | "is")
        }
        _ => false,
    }
}

/// Where a cursor-path jobs page starts.
#[must_use]
pub fn jobs_list_start(request: &PageRequest) -> ListStart<JobKey> {
    list_start(request, JOBS_CURSOR_SCOPE, parse_job_key)
}

/// Where a cursor-path runs page starts.
#[must_use]
pub fn runs_list_start(request: &PageRequest, scope_job_id: Option<&str>) -> ListStart<RunKey> {
    list_start(request, &runs_cursor_scope(scope_job_id), parse_run_key)
}

/// Build a jobs page from `limit + 1` rows read at `start`.
#[must_use]
pub fn jobs_page_from_rows(
    mut rows: Vec<Job>,
    request: &PageRequest,
    limit: u32,
    start: &ListStart<JobKey>,
    total_count: Option<u64>,
) -> Page<JobSummary> {
    let has_more = rows.len() > limit as usize;
    rows.truncate(limit as usize);
    let next_cursor = next_cursor(has_more, start, request, limit, JOBS_CURSOR_SCOPE, || {
        rows.last().map(|j| {
            format!(
                "{}|{}",
                j.created_at.to_rfc3339_opts(SecondsFormat::Nanos, true),
                j.job_id
            )
        })
    });
    Page {
        items: rows.into_iter().map(job_to_summary).collect(),
        has_more,
        total_count,
        ..Page::default()
    }
    .with_next_cursor(next_cursor)
}

/// Build a runs page from `limit + 1` rows read at `start`.
#[must_use]
pub fn runs_page_from_rows(
    mut rows: Vec<Run>,
    request: &PageRequest,
    scope_job_id: Option<&str>,
    limit: u32,
    start: &ListStart<RunKey>,
    total_count: Option<u64>,
) -> Page<RunSummary> {
    let has_more = rows.len() > limit as usize;
    rows.truncate(limit as usize);
    let scope = runs_cursor_scope(scope_job_id);
    let next_cursor = next_cursor(has_more, start, request, limit, &scope, || {
        rows.last().map(|r| {
            format!(
                "{}|{}",
                r.started_at.to_rfc3339_opts(SecondsFormat::Nanos, true),
                r.run_id
            )
        })
    });
    Page {
        items: rows.into_iter().map(run_to_summary).collect(),
        has_more,
        total_count,
        ..Page::default()
    }
    .with_next_cursor(next_cursor)
}

fn list_start<K>(request: &PageRequest, scope: &str, parse: fn(&str) -> Option<K>) -> ListStart<K> {
    let key =
        PageCursor::from_request(request, scope).and_then(|c| c.key.as_deref().and_then(parse));
    match key {
        Some(key) => ListStart::After(Some(key)),
        None if request.offset == 0 => ListStart::After(None),
        None => ListStart::Offset(request.offset as usize),
    }
}

fn next_cursor<K>(
    has_more: bool,
    start: &ListStart<K>,
    request: &PageRequest,
    limit: u32,
    scope: &str,
    last_key: impl FnOnce() -> Option<String>,
) -> Option<String> {
    if !has_more || matches!(start, ListStart::Offset(_)) {
        return None;
    }
    let position = u64::from(request.offset) + u64::from(limit);
    let fingerprint = request.query_fingerprint(scope);
    last_key().map(|key| PageCursor::with_key(position, key, fingerprint).encode())
}

fn split_key(key: &str) -> Option<(DateTime<Utc>, &str)> {
    let (at, id) = key.split_once('|')?;
    let at = DateTime::parse_from_rfc3339(at).ok()?.with_timezone(&Utc);
    Some((at, id))
}

fn parse_job_key(key: &str) -> Option<JobKey> {
    let (created_at, id) = split_key(key)?;
    validate_job_id(id).ok()?;
    Some(JobKey {
        created_at,
        id: id.to_string(),
    })
}

fn parse_run_key(key: &str) -> Option<RunKey> {
    let (started_at, id) = split_key(key)?;
    validate_run_id(id).ok()?;
    Some(RunKey {
        started_at,
        id: id.to_string(),
    })
}
