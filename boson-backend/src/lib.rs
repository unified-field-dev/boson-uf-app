//! Pure backend contracts for the Boson ops UI server surface.
//!
//! DTO shapes and pure mapping/validation helpers that `boson-app` `#[server]` functions
//! call after resolving Higgs and Boson coordinator request context. Keeps task, job, run,
//! and dashboard contracts unit-testable without a Leptos host or UI graph.
//!
//! ## Features
//!
//! - **Id validation** — Reject blank, oversized, or path-unsafe task names, job ids, and
//!   run ids before coordinator lookups. [Get started](#validate-ids)
//! - **Task/job/run mapping** — Pure helpers that build UI DTOs from coordinator jobs,
//!   runs, and registry task descriptors. [Get started](#map-task-job-run)
//! - **Dashboard aggregates** — KPI counters for tasks, queued/running jobs, and recent
//!   runs via [`dashboard_stats`]. [Get started](#dashboard-kpis)
//! - **Ops path encoding** — Percent-encode path segments for `/boson` hrefs via
//!   [`encode_ops_path_segment`], [`boson_task_path`], [`boson_run_path`], and related
//!   helpers.
//! - **DataTable query adapters** — Apply status and quick-search filters for queue and run
//!   tables via [`apply_jobs_datatable_query`] and [`apply_runs_datatable_query`].
//!
//! ## Validate ids
//!
//! Ops UI detail lookups reject ids that would break routing or leak path segments into
//! Boson IO. [`validate_task_name`], [`validate_job_id`], and [`validate_run_id`] run before
//! `boson-app` server functions call coordinator APIs — call them in custom wrappers when
//! you add new read paths that accept path or query parameters.
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
//! Mapping helpers turn coordinator jobs, runs, and registry descriptors into serde-friendly
//! DTOs the UI can render without touching Boson internals. [`task_summary_from_parts`] and
//! [`find_task_by_name`] back task list/detail pages; [`job_to_summary`] shapes queue rows;
//! [`run_to_summary`] builds run history previews.
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
//! Dashboard KPI aggregates provide registry size and active job counters without
//! UI-specific formatting. [`dashboard_stats`] packages task count, queued jobs, running
//! jobs, and runs started in the last 24 hours into [`DashboardStats`]; chart bucketing lives
//! in [`run_stats_series_from_runs`] after the caller loads run rows.
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
//! ## Examples ladder
//!
//! | Level | Where |
//! |-------|--------|
//! | Highlight | [Validate ids](#validate-ids) |
//! | Mid | This crate's unit + integ suites (`docs/VERIFICATION.md`) |
//! | Detailed | `examples/protected-boson-host` (auth + dashboard KPIs) |

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
    encode_ops_path_segment, validate_job_id, validate_run_id, validate_task_name, BosonIdError,
    MAX_BOSON_ID_CHARS,
};

#[cfg(test)]
#[path = "unit_tests.rs"]
mod tests;
