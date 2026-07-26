//! UI-facing DTOs and paging constants for Boson server contracts.

use serde::{Deserialize, Serialize};

/// Wire status for a queued or active Boson job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobStatusDto {
    /// Waiting to be claimed by a worker.
    Queued,
    /// Currently executing.
    Running,
    /// Finished successfully.
    Success,
    /// Finished with failure.
    Failed,
    /// Cancelled before or during execution.
    Canceled,
}

/// Wire status for a single Boson run attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RunStatusDto {
    /// Attempt currently in progress.
    Running,
    /// Attempt finished successfully.
    Success,
    /// Attempt finished with failure.
    Failed,
    /// Attempt cancelled.
    Canceled,
    /// Attempt exceeded its time budget.
    Timeout,
}

/// Summary of a registered task with effective config and aggregate stats.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskSummary {
    /// Registry task name.
    pub name: String,
    /// JSON-encoded task signature / schema.
    pub signature_json: String,
    /// Default priority from the registry descriptor.
    pub default_priority: i32,
    /// Default pool from the registry descriptor.
    pub default_pool: String,
    /// Effective priority after config overlay.
    pub effective_priority: i32,
    /// Effective pool after config overlay.
    pub effective_pool: String,
    /// Count of jobs currently queued for this task.
    pub jobs_queued: u32,
    /// Total runs recorded for this task.
    pub runs_total: u32,
    /// Success rate percent when `runs_total > 0`.
    pub success_rate_pct: Option<f64>,
}

/// Summary of a Boson job for queue / list views.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobSummary {
    /// Unique job identifier.
    pub job_id: String,
    /// Registry task name the job targets.
    pub task_name: String,
    /// Current job status.
    pub status: JobStatusDto,
    /// Scheduling priority.
    pub priority: i32,
    /// Execution pool name.
    pub pool: String,
    /// RFC3339 creation timestamp.
    pub created_at: String,
}

/// Summary of a Boson run for history / detail views.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunSummary {
    /// Unique run identifier.
    pub run_id: String,
    /// Parent job identifier.
    pub job_id: String,
    /// Registry task name.
    pub task_name: String,
    /// Run outcome status.
    pub status: RunStatusDto,
    /// Attempt number within the job.
    pub attempt: i32,
    /// RFC3339 start timestamp.
    pub started_at: String,
    /// RFC3339 finish timestamp, if finished.
    pub finished_at: Option<String>,
    /// Wall duration in milliseconds, if known.
    pub duration_ms: Option<i64>,
    /// Error message when the run failed.
    pub error_message: Option<String>,
}

/// Retry policy fields exposed to the UI.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetryPolicyDto {
    /// Maximum attempts including the first try.
    pub max_attempts: u32,
    /// Base backoff delay in milliseconds.
    pub base_delay_ms: u64,
    /// Multiplier applied between attempts.
    pub backoff_multiplier: f64,
    /// Cap on backoff delay in milliseconds.
    pub max_delay_ms: u64,
}

/// Task configuration DTO returned by get/update config endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskConfigDto {
    /// Registry task name.
    pub task_name: String,
    /// Effective priority.
    pub priority: i32,
    /// Effective pool.
    pub pool: String,
    /// Effective retry policy.
    pub retry_policy: RetryPolicyDto,
    /// RFC3339 last-updated timestamp.
    pub updated_at: String,
}

/// Aggregate counters shown on the Boson dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardStats {
    /// Registered task count.
    pub task_count: u32,
    /// Jobs currently queued.
    pub jobs_queued: u32,
    /// Jobs currently running.
    pub jobs_running: u32,
    /// Runs started in the last 24 hours.
    pub runs_today: u32,
}

/// Time-series point for dashboard charts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashboardChartPoint {
    /// Bucket timestamp (UTC).
    pub ts: chrono::DateTime<chrono::Utc>,
    /// Aggregated value for the bucket.
    pub value: f64,
}

/// Named time series for dashboard charts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashboardChartSeries {
    /// Stable series id (e.g. `"successful"`).
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// Ordered points spanning the selected range.
    pub points: Vec<DashboardChartPoint>,
}

/// Partial update request for task configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateTaskConfigRequest {
    /// New priority when set.
    pub priority: Option<i32>,
    /// New pool when set.
    pub pool: Option<String>,
    /// New retry policy when set.
    pub retry_policy: Option<RetryPolicyDto>,
}

/// Gluon virtual pool row for Boson task-config pool picker.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GluonPoolPickRow {
    /// Virtual pool id (use as Boson `pool` string).
    pub id: String,
    /// Display label.
    pub label: String,
    /// Secondary detail text.
    pub detail: String,
}

/// Page size used by the tasks infinite scroll / `DataTable`.
pub const TASKS_PAGE_SIZE: u32 = 20;
/// Page size used by the jobs/queue infinite scroll / `DataTable`.
pub const JOBS_PAGE_SIZE: u32 = 20;
/// Page size used by the runs infinite scroll / `DataTable`.
pub const RUNS_PAGE_SIZE: u32 = 20;

/// Upper bound for in-memory list fetches (`DataTable` filters, chart series).
pub const BOSON_LIST_FETCH_CAP: usize = 50_000;
