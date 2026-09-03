//! Pure backend contracts for the Boson ops UI server surface.
//!
//! DTO shapes and pure mapping/validation helpers that `boson-app` `#[server]` functions
//! call after resolving Higgs and Boson coordinator request context. Keeps task, job, run,
//! and dashboard contracts unit-testable without a Leptos host or UI graph.
//!
//! ## Features
//!
//! - **Id validation** — Validates task names, job ids, and run ids so blank, oversized, or
//!   path-unsafe values fail closed before coordinator lookups. [Get started](#validate-ids)
//! - **Task/job/run mapping** — Builds UI DTOs from coordinator jobs, runs, and registry
//!   task descriptors without performing IO. [Get started](#map-task-job-run)
//! - **Dashboard aggregates** — Provides KPI counters for tasks, queued/running jobs, and
//!   recent runs via [`dashboard_stats`]. [Get started](#dashboard-kpis)
//! - **Input validation** — Rejects chart windows and task-config updates outside the ops UI
//!   bounds via [`validate_range_secs`] and [`validate_task_config_update`], and formats
//!   fail-closed config-load errors via [`format_task_config_load_error`].
//!   [Get started](#validate-input)
//! - **Ops path encoding** — Builds percent-encoded path segments for `/boson` hrefs via
//!   [`encode_ops_path_segment`], [`boson_task_path`], [`boson_run_path`], and related
//!   helpers.
//! - **`DataTable` query adapters** — Supports status and quick-search filters for queue and
//!   run tables via [`apply_jobs_datatable_query`] and [`apply_runs_datatable_query`].
//!
//! ## Validate ids
//!
//! Id validation checks path and query parameters before they reach Boson IO, so blank or
//! path-unsafe values fail closed instead of breaking routing. [`validate_task_name`],
//! [`validate_job_id`], and [`validate_run_id`] run in `boson-app` server functions ahead of
//! coordinator lookups — call them in custom wrappers when you add new read paths that
//! accept path or query parameters.
//!
//! **Prerequisites:** None beyond importing this crate; validators are synchronous and
//! return [`Result<(), BosonIdError>`].
//!
//! ```rust,ignore
//! use boson_backend::{
//!     validate_task_name, validate_job_id, validate_run_id, BosonIdError,
//! };
//!
//! validate_task_name("orders.task").expect("valid task");
//! assert_eq!(
//!     validate_task_name("").unwrap_err(),
//!     BosonIdError::EmptyTaskName
//! );
//! validate_job_id("job-1").expect("valid job");
//! validate_run_id("run-1").expect("valid run");
//! ```
//!
//! On success validators return `Ok(())` and the trimmed id is safe for lookup. Blank,
//! oversized, control-character, slash, backslash, or `.` / `..` names map to typed
//! [`BosonIdError`] variants with operator-facing messages.
//!
//! ## Map task job run
//!
//! Task/job/run mapping turns coordinator jobs, runs, and registry descriptors into
//! serde-friendly DTOs the UI can render without touching Boson internals.
//! [`task_summary_from_parts`] and [`find_task_by_name`] back task list/detail pages;
//! [`job_to_summary`] shapes queue rows; [`run_to_summary`] builds run history previews.
//! Call these after you already hold coordinator rows in memory — typically inside
//! `boson-app` `#[server]` handlers that assemble list or detail responses.
//!
//! **Prerequisites:** Caller already loaded jobs, runs, and task config from the coordinator
//! — these functions do not perform IO.
//!
//! ```rust,ignore
//! use boson_backend::{
//!     task_summary_from_parts, job_to_summary, run_to_summary, find_task_by_name, TaskStatsAgg,
//! };
//! use boson_core::{Job, JobStatus, Run, RunStatus, TaskConfig};
//! use chrono::Utc;
//!
//! let config = TaskConfig::default_for("orders.task");
//! let stats = TaskStatsAgg {
//!     jobs_queued: 1,
//!     runs_total: 4,
//!     success_count: 3,
//! };
//! let task = task_summary_from_parts("orders.task", "{}", 0, "global", &config, stats);
//! assert_eq!(task.name, "orders.task");
//!
//! let job = Job {
//!     job_id: "job-1".into(),
//!     task_name: "orders.task".into(),
//!     actor_json: serde_json::json!({}),
//!     params_json: serde_json::json!({}),
//!     priority: 1,
//!     pool: "global".into(),
//!     status: JobStatus::Queued,
//!     idempotency_key: None,
//!     created_at: Utc::now(),
//!     signature_hash: 0,
//!     attempt: 1,
//! };
//! let summary = job_to_summary(job);
//! assert_eq!(summary.task_name, "orders.task");
//!
//! let run = Run {
//!     run_id: "run-1".into(),
//!     job_id: "job-1".into(),
//!     task_name: "orders.task".into(),
//!     attempt: 1,
//!     status: RunStatus::Success,
//!     started_at: Utc::now(),
//!     finished_at: None,
//!     duration_ms: None,
//!     error_message: None,
//! };
//! let run_row = run_to_summary(run);
//! assert_eq!(run_row.task_name, "orders.task");
//!
//! let tasks = vec![task];
//! assert_eq!(
//!     find_task_by_name(&tasks, "orders.task").map(|t| t.name.as_str()),
//!     Some("orders.task")
//! );
//! ```
//!
//! On success helpers return populated [`TaskSummary`], [`JobSummary`], or [`RunSummary`]
//! rows ready for JSON serialization. Lookup helpers return `None` when the name or id is
//! absent from the caller-provided slice.
//!
//! ## Dashboard KPIs
//!
//! Dashboard aggregates package registry size and active job counters into a single
//! [`DashboardStats`] value for the ops landing page, without UI-specific formatting.
//! [`dashboard_stats`] takes task count, queued jobs, running jobs, and runs started in the
//! last 24 hours; chart bucketing lives in [`run_stats_series_from_runs`] after the caller
//! loads run rows. Call this when a dashboard server function has already counted those
//! slices from the coordinator.
//!
//! **Prerequisites:** Caller supplies counts from coordinator queries — these helpers do not
//! call Boson.
//!
//! ```rust,ignore
//! use boson_backend::{dashboard_stats, DashboardStats};
//!
//! let stats: DashboardStats = dashboard_stats(3, 5, 1, 12);
//! assert_eq!(stats.task_count, 3);
//! assert_eq!(stats.jobs_queued, 5);
//! assert_eq!(stats.jobs_running, 1);
//! assert_eq!(stats.runs_today, 12);
//! ```
//!
//! On success `stats` carries the four KPI fields consumed by `boson-app` dashboard server
//! functions.
//!
//! ## Validate input
//!
//! Input validation covers dashboard chart windows and partial task-config updates before
//! coordinator writes, so out-of-range priority, unsafe pool names, or unsupported
//! `range_secs` fail with typed [`BosonInputError`] instead of reaching Boson IO.
//! [`validate_range_secs`] accepts only the 24h and 7d windows the UI exposes;
//! [`validate_task_config_update`] checks set fields on [`UpdateTaskConfigRequest`].
//! When config load itself fails, [`format_task_config_load_error`] builds the operator
//! message used by `boson-app` so summaries never fall back to silent defaults.
//!
//! **Prerequisites:** None beyond this crate; validators are synchronous. Call them from
//! `#[server]` handlers (or custom wrappers) before coordinator upserts or chart queries.
//!
//! ```rust,ignore
//! use boson_backend::{
//!     format_task_config_load_error, validate_range_secs, validate_task_config_update,
//!     BosonInputError, UpdateTaskConfigRequest, RANGE_SECS_24H,
//! };
//!
//! validate_range_secs(RANGE_SECS_24H).expect("24h window");
//! assert_eq!(
//!     validate_range_secs(0).unwrap_err(),
//!     BosonInputError::InvalidRangeSecs
//! );
//!
//! let req = UpdateTaskConfigRequest {
//!     priority: Some(10),
//!     pool: Some("global".into()),
//!     ..Default::default()
//! };
//! validate_task_config_update(&req).expect("in range");
//! assert_eq!(
//!     validate_task_config_update(&UpdateTaskConfigRequest {
//!         priority: Some(i32::MAX),
//!         ..Default::default()
//!     })
//!     .unwrap_err(),
//!     BosonInputError::PriorityOutOfRange
//! );
//!
//! let msg = format_task_config_load_error("coordinator unavailable");
//! assert!(msg.starts_with("Failed to load task config:"));
//! ```
//!
//! On success validators return `Ok(())`. Failures map to [`BosonInputError`] variants with
//! stable Display text (`Invalid range_secs:…`, `Invalid task config update:…`). Config-load
//! formatting always prefixes `Failed to load task config:`.
//!
//! ## Examples
//!
//! Start with [Validate ids](#validate-ids). This crate's unit and integ suites are listed in
//! `docs/VERIFICATION.md`. Runnable host: `examples/protected-boson-host` (auth + dashboard KPIs).

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
pub use validate::{
    boson_run_path, boson_runs_job_filter_path, boson_task_config_path, boson_task_path,
    encode_ops_path_segment, format_task_config_load_error, validate_job_id, validate_range_secs,
    validate_run_id, validate_task_config_update, validate_task_name, BosonIdError,
    BosonInputError, MAX_BOSON_ID_CHARS, MAX_POOL_NAME_CHARS, MAX_RETRY_ATTEMPTS,
    MAX_RETRY_DELAY_MS, MAX_TASK_PRIORITY, MIN_TASK_PRIORITY, RANGE_SECS_24H, RANGE_SECS_7D,
};

#[cfg(test)]
#[path = "unit_tests.rs"]
mod tests;
