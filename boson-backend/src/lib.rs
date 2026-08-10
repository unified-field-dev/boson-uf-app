//! Pure backend contracts for the Boson UF app server surface.
//!
//! Leptos `#[server]` entrypoints in `boson-app` resolve Higgs / coordinator
//! request context, then call these helpers so job, run, task, and dashboard
//! shapes stay unit- and integration-testable without a full host or UI graph.
//!
//! ## Organized by task
//!
//! | Task | Start here |
//! |------|------------|
//! | **Validate list/detail ids** | [`BosonIdError`], [`validate_task_name`], [`validate_job_id`], [`validate_run_id`] |
//! | **Task list/detail mapping** | [`TaskSummary`], [`find_task_by_name`], [`sort_tasks_by_name`], [`filter_tasks_by_query`] |
//! | **Job / queue mapping** | [`JobSummary`], [`find_job_by_id`], [`job_to_summary`], [`parse_job_status_filter`] |
//! | **Run list/detail mapping** | [`RunSummary`], [`find_run_by_id`], [`run_to_summary`] |
//! | **Task config update** | [`TaskConfigDto`], [`apply_task_config_update`], [`task_config_to_dto`] |
//! | **Dashboard KPIs / trends** | [`DashboardStats`], [`dashboard_stats`], [`run_stats_series_from_runs`] |
//! | **`DataTable` query adapters** | [`apply_jobs_datatable_query`], [`apply_runs_datatable_query`] |
//! | **UI pages / `#[server]` wrappers** | `boson-app` (not this crate) |
//!
//! ## Owns / does not own
//!
//! **Owns:** DTO shapes and pure mapping/validation helpers used by the Boson
//! ops UI server surface.
//!
//! **Does not own:** Leptos pages, Higgs `#[server]` wrappers, or route registration
//! (`boson-app`); Boson coordinator execution or `IsolatedLab` persistence (Boson core).
//!
//! ## Concern → API
//!
//! | Concern | API | Owner |
//! |---------|-----|-------|
//! | Id / name validation | [`BosonIdError`], [`validate_task_name`], [`validate_job_id`], [`validate_run_id`] | this crate |
//! | Task summaries / filters | [`TaskSummary`], [`find_task_by_name`], [`sort_tasks_by_name`], [`filter_tasks_by_query`] | this crate |
//! | Job summaries / status filter | [`JobSummary`], [`find_job_by_id`], [`job_to_summary`], [`parse_job_status_filter`] | this crate |
//! | Run summaries | [`RunSummary`], [`find_run_by_id`], [`run_to_summary`] | this crate |
//! | Task config DTOs | [`TaskConfigDto`], [`apply_task_config_update`], [`task_config_to_dto`] | this crate |
//! | Dashboard aggregates | [`DashboardStats`], [`dashboard_stats`], [`run_stats_series_from_runs`] | this crate |
//! | `DataTable` adapters | [`apply_jobs_datatable_query`], [`apply_runs_datatable_query`] | this crate |
//! | Pages, routes, server fns | `boson-app` (`BosonRoutes`) | `boson-app` |
//!
//! ## Examples ladder
//!
//! | Level | Where |
//! |-------|--------|
//! | Highlight | Concern → API table above |
//! | Mid | This crate's unit + integ suites (`docs/VERIFICATION.md`) |
//! | Detailed | `examples/protected-boson-host` (inventory `boson` / `/boson`; copy README) |

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod dashboard;
mod lookup;
mod map;
mod page_query;
mod types;
mod validate;

pub use dashboard::{
    align_run_bucket, fill_bucket_range, run_bucket_granularity, run_stats_series_from_runs,
    RunBucketGranularity,
};
pub use lookup::{
    filter_tasks_by_query, find_job_by_id, find_run_by_id, find_task_by_name, sort_tasks_by_name,
};
pub use map::{
    aggregate_task_stats, apply_task_config_update, dashboard_stats, default_gluon_pool_rows,
    job_status_to_dto, job_to_summary, parse_job_status_filter, retry_policy_from_dto,
    retry_policy_to_dto, run_status_to_dto, run_to_summary, success_rate_pct, task_config_to_dto,
    task_summary_from_parts, TaskStatsAgg,
};
pub use page_query::{
    apply_jobs_datatable_query, apply_runs_datatable_query, extract_status_filter, job_status_key,
    quick_search_text, resolve_job_filter, run_status_key,
};
pub use types::{
    clamp_page_list_limit, DashboardChartPoint, DashboardChartSeries, DashboardStats,
    GluonPoolPickRow, JobStatusDto, JobSummary, RetryPolicyDto, RunStatusDto, RunSummary,
    TaskConfigDto, TaskSummary, UpdateTaskConfigRequest, BOSON_LIST_FETCH_CAP, JOBS_PAGE_SIZE,
    MAX_PAGE_LIST_LIMIT, RUNS_PAGE_SIZE, TASKS_PAGE_SIZE,
};
pub use validate::{validate_job_id, validate_run_id, validate_task_name, BosonIdError};

#[cfg(test)]
#[path = "unit_tests.rs"]
mod tests;
