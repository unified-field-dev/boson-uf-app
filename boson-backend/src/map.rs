//! DTO mappers and pure aggregates backing Boson job/run/task server fns.

use std::collections::HashMap;

use boson_core::{Job, JobStatus, RetryPolicy, Run, RunStatus, TaskConfig};
use chrono::{DateTime, Utc};

use crate::types::{
    DashboardStats, JobStatusDto, JobSummary, RetryPolicyDto, RunStatusDto, RunSummary,
    TaskConfigDto, TaskSummary, UpdateTaskConfigRequest,
};

/// Per-task aggregates built from full job/run lists (`get_tasks`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TaskStatsAgg {
    /// Count of jobs in [`JobStatus::Queued`] for the task.
    pub jobs_queued: u32,
    /// Total runs for the task.
    pub runs_total: u32,
    /// Runs with [`RunStatus::Success`].
    pub success_count: u32,
}

/// Maps a core job status onto the UI wire enum.
#[must_use]
pub const fn job_status_to_dto(s: JobStatus) -> JobStatusDto {
    match s {
        JobStatus::Queued => JobStatusDto::Queued,
        JobStatus::Running => JobStatusDto::Running,
        JobStatus::Success => JobStatusDto::Success,
        JobStatus::Failed => JobStatusDto::Failed,
        JobStatus::Canceled => JobStatusDto::Canceled,
    }
}

/// Maps a core run status onto the UI wire enum.
#[must_use]
pub const fn run_status_to_dto(s: RunStatus) -> RunStatusDto {
    match s {
        RunStatus::Running => RunStatusDto::Running,
        RunStatus::Success => RunStatusDto::Success,
        RunStatus::Failed => RunStatusDto::Failed,
        RunStatus::Canceled => RunStatusDto::Canceled,
        RunStatus::Timeout => RunStatusDto::Timeout,
    }
}

/// Maps a core retry policy onto the UI DTO.
#[must_use]
pub const fn retry_policy_to_dto(r: &RetryPolicy) -> RetryPolicyDto {
    RetryPolicyDto {
        max_attempts: r.max_attempts,
        base_delay_ms: r.base_delay_ms,
        backoff_multiplier: r.backoff_multiplier,
        max_delay_ms: r.max_delay_ms,
    }
}

/// Maps a UI retry policy DTO onto the core type.
#[must_use]
pub const fn retry_policy_from_dto(r: &RetryPolicyDto) -> RetryPolicy {
    RetryPolicy {
        max_attempts: r.max_attempts,
        base_delay_ms: r.base_delay_ms,
        backoff_multiplier: r.backoff_multiplier,
        max_delay_ms: r.max_delay_ms,
    }
}

/// Parses lowercase job-status filter strings used by queue list endpoints.
///
/// Unknown / empty / wrong-case values yield [`None`] (no filter).
#[must_use]
pub fn parse_job_status_filter(s: &str) -> Option<JobStatus> {
    match s {
        "queued" => Some(JobStatus::Queued),
        "running" => Some(JobStatus::Running),
        "success" => Some(JobStatus::Success),
        "failed" => Some(JobStatus::Failed),
        "canceled" => Some(JobStatus::Canceled),
        _ => None,
    }
}

/// Maps a core [`Job`] onto a list-row [`JobSummary`].
#[must_use]
pub fn job_to_summary(j: Job) -> JobSummary {
    JobSummary {
        job_id: j.job_id,
        task_name: j.task_name,
        status: job_status_to_dto(j.status),
        priority: j.priority,
        pool: j.pool,
        created_at: j.created_at.to_rfc3339(),
    }
}

/// Maps a core [`Run`] onto a list-row [`RunSummary`].
#[must_use]
pub fn run_to_summary(r: Run) -> RunSummary {
    RunSummary {
        run_id: r.run_id,
        job_id: r.job_id,
        task_name: r.task_name,
        status: run_status_to_dto(r.status),
        attempt: r.attempt,
        started_at: r.started_at.to_rfc3339(),
        finished_at: r.finished_at.map(|dt| dt.to_rfc3339()),
        duration_ms: r.duration_ms,
        error_message: r.error_message,
    }
}

/// Success rate percent when there is at least one run; otherwise [`None`].
#[must_use]
pub fn success_rate_pct(runs_total: u32, success_count: u32) -> Option<f64> {
    if runs_total > 0 {
        Some((f64::from(success_count) / f64::from(runs_total)) * 100.0)
    } else {
        None
    }
}

/// Builds a [`TaskSummary`] from descriptor fields, config, and aggregates.
#[must_use]
pub fn task_summary_from_parts(
    name: &str,
    signature_json: &str,
    default_priority: i32,
    default_pool: &str,
    config: &TaskConfig,
    stats: TaskStatsAgg,
) -> TaskSummary {
    TaskSummary {
        name: name.to_string(),
        signature_json: signature_json.to_string(),
        default_priority,
        default_pool: default_pool.to_string(),
        effective_priority: config.priority,
        effective_pool: config.pool.clone(),
        jobs_queued: stats.jobs_queued,
        runs_total: stats.runs_total,
        success_rate_pct: success_rate_pct(stats.runs_total, stats.success_count),
    }
}

/// Aggregates per-task queued-job and run-success counters.
#[must_use]
pub fn aggregate_task_stats(jobs: &[Job], runs: &[Run]) -> HashMap<String, TaskStatsAgg> {
    let mut out: HashMap<String, TaskStatsAgg> = HashMap::new();

    for job in jobs {
        let entry = out.entry(job.task_name.clone()).or_default();
        if job.status == JobStatus::Queued {
            entry.jobs_queued += 1;
        }
    }

    for run in runs {
        let entry = out.entry(run.task_name.clone()).or_default();
        entry.runs_total += 1;
        if run.status == RunStatus::Success {
            entry.success_count += 1;
        }
    }

    out
}

/// Maps a core [`TaskConfig`] onto [`TaskConfigDto`].
#[must_use]
pub fn task_config_to_dto(config: &TaskConfig) -> TaskConfigDto {
    TaskConfigDto {
        task_name: config.task_name.clone(),
        priority: config.priority,
        pool: config.pool.clone(),
        retry_policy: retry_policy_to_dto(&config.retry_policy),
        updated_at: config.updated_at.to_rfc3339(),
    }
}

/// Applies a partial UI update onto a mutable [`TaskConfig`].
pub fn apply_task_config_update(
    config: &mut TaskConfig,
    req: &UpdateTaskConfigRequest,
    now: DateTime<Utc>,
) {
    if let Some(p) = req.priority {
        config.priority = p;
    }
    if let Some(ref p) = req.pool {
        config.pool.clone_from(p);
    }
    if let Some(ref r) = req.retry_policy {
        config.retry_policy = retry_policy_from_dto(r);
    }
    config.updated_at = now;
}

/// Builds dashboard KPI counters after coordinator queries resolve.
#[must_use]
pub const fn dashboard_stats(
    task_count: u32,
    jobs_queued: u32,
    jobs_running: u32,
    runs_today: u32,
) -> DashboardStats {
    DashboardStats {
        task_count,
        jobs_queued,
        jobs_running,
        runs_today,
    }
}

/// Default Gluon pool picker row used until Wave 7 integration lands.
#[must_use]
pub fn default_gluon_pool_rows() -> Vec<crate::types::GluonPoolPickRow> {
    vec![crate::types::GluonPoolPickRow {
        id: "global".to_string(),
        label: "global (default)".to_string(),
        detail: "Default in-process pool name when no Gluon pool is used.".to_string(),
    }]
}
