//! UI-facing DTOs and paging constants for Boson server functions.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobStatusDto {
    Queued,
    Running,
    Success,
    Failed,
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RunStatusDto {
    Running,
    Success,
    Failed,
    Canceled,
    Timeout,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskSummary {
    pub name: String,
    pub signature_json: String,
    pub default_priority: i32,
    pub default_pool: String,
    pub effective_priority: i32,
    pub effective_pool: String,
    pub jobs_queued: u32,
    pub runs_total: u32,
    pub success_rate_pct: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobSummary {
    pub job_id: String,
    pub task_name: String,
    pub status: JobStatusDto,
    pub priority: i32,
    pub pool: String,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunSummary {
    pub run_id: String,
    pub job_id: String,
    pub task_name: String,
    pub status: RunStatusDto,
    pub attempt: i32,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicyDto {
    pub max_attempts: u32,
    pub base_delay_ms: u64,
    pub backoff_multiplier: f64,
    pub max_delay_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfigDto {
    pub task_name: String,
    pub priority: i32,
    pub pool: String,
    pub retry_policy: RetryPolicyDto,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub task_count: u32,
    pub jobs_queued: u32,
    pub jobs_running: u32,
    pub runs_today: u32,
}

/// Time-series point for dashboard charts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardChartPoint {
    pub ts: chrono::DateTime<chrono::Utc>,
    pub value: f64,
}

/// Named time series for dashboard charts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardChartSeries {
    pub id: String,
    pub label: String,
    pub points: Vec<DashboardChartPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskConfigRequest {
    pub priority: Option<i32>,
    pub pool: Option<String>,
    pub retry_policy: Option<RetryPolicyDto>,
}

/// Gluon virtual pool row for Boson task-config pool picker.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GluonPoolPickRow {
    /// Virtual pool id (use as Boson `pool` string).
    pub id: String,
    pub label: String,
    pub detail: String,
}

/// Page size used by the tasks infinite scroll / DataTable.
pub const TASKS_PAGE_SIZE: u32 = 20;
/// Page size used by the jobs/queue infinite scroll / DataTable.
pub const JOBS_PAGE_SIZE: u32 = 20;
/// Page size used by the runs infinite scroll / DataTable.
pub const RUNS_PAGE_SIZE: u32 = 20;

/// Upper bound for in-memory list fetches (DataTable filters, chart series). 0.1.n limitation.
pub const BOSON_LIST_FETCH_CAP: usize = 50_000;
