//! `PageRequest` helpers for Boson `DataTable` server adapters.

use orbital_data::DataValue;
use orbital_paging::{FilterLogicWire, FilterQuery, FilterRuleParam, PageRequest};

use crate::types::{JobStatusDto, JobSummary, RunStatusDto, RunSummary};

fn filter_rule_text(value: &DataValue) -> String {
    value.display_string()
}

fn text_contains(haystack: &str, needle: &str) -> bool {
    let needle = needle.trim();
    if needle.is_empty() {
        return true;
    }
    haystack.to_lowercase().contains(&needle.to_lowercase())
}

fn text_equals(left: &str, right: &str) -> bool {
    left.trim().eq_ignore_ascii_case(right.trim())
}

/// Lowercase status key for a run wire status.
#[must_use]
pub const fn run_status_key(status: RunStatusDto) -> &'static str {
    match status {
        RunStatusDto::Running => "running",
        RunStatusDto::Success => "success",
        RunStatusDto::Failed => "failed",
        RunStatusDto::Canceled => "canceled",
        RunStatusDto::Timeout => "timeout",
    }
}

/// Lowercase status key for a job wire status.
#[must_use]
pub const fn job_status_key(status: JobStatusDto) -> &'static str {
    match status {
        JobStatusDto::Queued => "queued",
        JobStatusDto::Running => "running",
        JobStatusDto::Success => "success",
        JobStatusDto::Failed => "failed",
        JobStatusDto::Canceled => "canceled",
    }
}

/// Status filter for queue/jobs (`status` column equals rule).
#[must_use]
pub fn extract_status_filter(request: &PageRequest) -> Option<String> {
    let filter = request.filter.as_ref()?;
    filter
        .items
        .iter()
        .find(|rule| rule.field == "status" && matches!(rule.operator.as_str(), "equals" | "is"))
        .map(|rule| filter_rule_text(&rule.value).to_lowercase())
}

/// Job id from URL scope or structured filter on `job_id`.
#[must_use]
pub fn resolve_job_filter(scope_job: Option<String>, request: &PageRequest) -> Option<String> {
    if let Some(job) = scope_job.filter(|s| !s.is_empty()) {
        return Some(job);
    }
    let filter = request.filter.as_ref()?;
    filter
        .items
        .iter()
        .find(|rule| rule.field == "job_id" && matches!(rule.operator.as_str(), "equals" | "is"))
        .map(|rule| filter_rule_text(&rule.value))
}

/// Trimmed quick-search text, or [`None`] when blank.
#[must_use]
pub fn quick_search_text(request: &PageRequest) -> Option<String> {
    request
        .quick_search
        .as_ref()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn run_matches_filter_rule(run: &RunSummary, rule: &FilterRuleParam) -> bool {
    let value = filter_rule_text(&rule.value);
    match rule.field.as_str() {
        "run_id" => match rule.operator.as_str() {
            "contains" | "not_contains" => text_contains(&run.run_id, &value),
            "equals" | "is" => text_equals(&run.run_id, &value),
            "not_equals" | "is_not" => !text_equals(&run.run_id, &value),
            _ => true,
        },
        "job_id" => match rule.operator.as_str() {
            "contains" | "not_contains" => text_contains(&run.job_id, &value),
            "equals" | "is" => text_equals(&run.job_id, &value),
            "not_equals" | "is_not" => !text_equals(&run.job_id, &value),
            _ => true,
        },
        "task_name" => match rule.operator.as_str() {
            "contains" | "not_contains" => text_contains(&run.task_name, &value),
            "equals" | "is" => text_equals(&run.task_name, &value),
            _ => true,
        },
        "status" => match rule.operator.as_str() {
            "equals" | "is" => text_equals(run_status_key(run.status), &value),
            "not_equals" | "is_not" => !text_equals(run_status_key(run.status), &value),
            _ => true,
        },
        _ => true,
    }
}

fn apply_run_filter_query(runs: &mut Vec<RunSummary>, filter: &FilterQuery) {
    runs.retain(|run| {
        let matches: Vec<bool> = filter
            .items
            .iter()
            .map(|rule| run_matches_filter_rule(run, rule))
            .collect();
        match filter.logic {
            FilterLogicWire::And => matches.iter().all(|m| *m),
            FilterLogicWire::Or => matches.iter().any(|m| *m),
        }
    });
}

/// Applies quick-search + structured filters to a runs page payload.
pub fn apply_runs_datatable_query(runs: &mut Vec<RunSummary>, request: &PageRequest) {
    if let Some(ref q) = quick_search_text(request) {
        let q_lower = q.to_lowercase();
        runs.retain(|r| {
            r.run_id.to_lowercase().contains(&q_lower)
                || r.job_id.to_lowercase().contains(&q_lower)
                || r.task_name.to_lowercase().contains(&q_lower)
        });
    }
    if let Some(ref filter) = request.filter {
        apply_run_filter_query(runs, filter);
    }
}

fn job_matches_filter_rule(job: &JobSummary, rule: &FilterRuleParam) -> bool {
    let value = filter_rule_text(&rule.value);
    match rule.field.as_str() {
        "job_id" => match rule.operator.as_str() {
            "contains" | "not_contains" => text_contains(&job.job_id, &value),
            "equals" | "is" => text_equals(&job.job_id, &value),
            _ => true,
        },
        "task_name" => match rule.operator.as_str() {
            "contains" | "not_contains" => text_contains(&job.task_name, &value),
            "equals" | "is" => text_equals(&job.task_name, &value),
            _ => true,
        },
        "status" => match rule.operator.as_str() {
            "equals" | "is" => text_equals(job_status_key(job.status), &value),
            "not_equals" | "is_not" => !text_equals(job_status_key(job.status), &value),
            _ => true,
        },
        "pool" => match rule.operator.as_str() {
            "contains" | "not_contains" => text_contains(&job.pool, &value),
            "equals" | "is" => text_equals(&job.pool, &value),
            _ => true,
        },
        _ => true,
    }
}

fn apply_job_filter_query(jobs: &mut Vec<JobSummary>, filter: &FilterQuery) {
    jobs.retain(|job| {
        let matches: Vec<bool> = filter
            .items
            .iter()
            .map(|rule| job_matches_filter_rule(job, rule))
            .collect();
        match filter.logic {
            FilterLogicWire::And => matches.iter().all(|m| *m),
            FilterLogicWire::Or => matches.iter().any(|m| *m),
        }
    });
}

/// Applies quick-search + structured filters to a jobs page payload.
pub fn apply_jobs_datatable_query(jobs: &mut Vec<JobSummary>, request: &PageRequest) {
    if let Some(ref q) = quick_search_text(request) {
        let q_lower = q.to_lowercase();
        jobs.retain(|j| {
            j.job_id.to_lowercase().contains(&q_lower)
                || j.task_name.to_lowercase().contains(&q_lower)
                || j.pool.to_lowercase().contains(&q_lower)
        });
    }
    if let Some(ref filter) = request.filter {
        apply_job_filter_query(jobs, filter);
    }
}
