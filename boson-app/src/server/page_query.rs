//! PageRequest helpers for Boson DataTable server adapters.

use orbital_data::DataValue;
use orbital_paging::{FilterLogicWire, FilterQuery, FilterRuleParam, PageRequest};

use super::types::{JobStatusDto, JobSummary, RunStatusDto, RunSummary};

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

pub fn run_status_key(status: RunStatusDto) -> &'static str {
    match status {
        RunStatusDto::Running => "running",
        RunStatusDto::Success => "success",
        RunStatusDto::Failed => "failed",
        RunStatusDto::Canceled => "canceled",
        RunStatusDto::Timeout => "timeout",
    }
}

pub fn job_status_key(status: JobStatusDto) -> &'static str {
    match status {
        JobStatusDto::Queued => "queued",
        JobStatusDto::Running => "running",
        JobStatusDto::Success => "success",
        JobStatusDto::Failed => "failed",
        JobStatusDto::Canceled => "canceled",
    }
}

/// Status filter for queue/jobs (`status` column equals rule).
pub fn extract_status_filter(request: &PageRequest) -> Option<String> {
    let filter = request.filter.as_ref()?;
    filter
        .items
        .iter()
        .find(|rule| {
            rule.field == "status" && matches!(rule.operator.as_str(), "equals" | "is")
        })
        .map(|rule| filter_rule_text(&rule.value).to_lowercase())
}

/// Job id from URL scope or structured filter on `job_id`.
pub fn resolve_job_filter(scope_job: Option<String>, request: &PageRequest) -> Option<String> {
    if let Some(job) = scope_job.filter(|s| !s.is_empty()) {
        return Some(job);
    }
    let filter = request.filter.as_ref()?;
    filter
        .items
        .iter()
        .find(|rule| {
            rule.field == "job_id" && matches!(rule.operator.as_str(), "equals" | "is")
        })
        .map(|rule| filter_rule_text(&rule.value))
}

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

#[cfg(test)]
mod tests {
    use orbital_data::DataValue;
    use orbital_paging::{FilterLogicWire, FilterQuery, FilterRuleParam, PageRequest};

    use super::*;
    use crate::server::types::{JobStatusDto, JobSummary, RunStatusDto, RunSummary};

    fn sample_job(id: &str, task: &str, status: JobStatusDto) -> JobSummary {
        JobSummary {
            job_id: id.into(),
            task_name: task.into(),
            status,
            priority: 0,
            pool: "global".into(),
            created_at: "2026-01-01T00:00:00Z".into(),
        }
    }

    fn sample_run(id: &str, job: &str, task: &str, status: RunStatusDto) -> RunSummary {
        RunSummary {
            run_id: id.into(),
            job_id: job.into(),
            task_name: task.into(),
            status,
            attempt: 1,
            started_at: "2026-01-01T00:00:00Z".into(),
            finished_at: None,
            duration_ms: None,
            error_message: None,
        }
    }

    #[test]
    fn job_status_key_maps_variants() {
        assert_eq!(job_status_key(JobStatusDto::Queued), "queued");
        assert_eq!(job_status_key(JobStatusDto::Running), "running");
    }

    #[test]
    fn run_status_key_maps_variants() {
        assert_eq!(run_status_key(RunStatusDto::Timeout), "timeout");
    }

    #[test]
    fn extract_status_filter_reads_equals_rule() {
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: None,
            filter: Some(FilterQuery {
                logic: FilterLogicWire::And,
                items: vec![FilterRuleParam {
                    field: "status".into(),
                    operator: "equals".into(),
                    value: DataValue::Text("queued".into()),
                }],
            }),
            sort: None,
        };
        assert_eq!(extract_status_filter(&request), Some("queued".into()));
    }

    #[test]
    fn resolve_job_filter_prefers_scope() {
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: None,
            filter: Some(FilterQuery {
                logic: FilterLogicWire::And,
                items: vec![FilterRuleParam {
                    field: "job_id".into(),
                    operator: "equals".into(),
                    value: DataValue::Text("from-filter".into()),
                }],
            }),
            sort: None,
        };
        assert_eq!(
            resolve_job_filter(Some("from-scope".into()), &request),
            Some("from-scope".into())
        );
    }

    #[test]
    fn quick_search_text_trims_and_rejects_empty() {
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: Some("  ".into()),
            filter: None,
            sort: None,
        };
        assert_eq!(quick_search_text(&request), None);
    }

    #[test]
    fn resolve_job_filter_falls_back_to_filter_rule() {
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: None,
            filter: Some(FilterQuery {
                logic: FilterLogicWire::And,
                items: vec![FilterRuleParam {
                    field: "job_id".into(),
                    operator: "equals".into(),
                    value: DataValue::Text("from-filter".into()),
                }],
            }),
            sort: None,
        };
        assert_eq!(
            resolve_job_filter(None, &request),
            Some("from-filter".into())
        );
        assert_eq!(
            resolve_job_filter(Some(String::new()), &request),
            Some("from-filter".into())
        );
    }

    #[test]
    fn extract_status_filter_ignores_non_status_rules() {
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: None,
            filter: Some(FilterQuery {
                logic: FilterLogicWire::And,
                items: vec![FilterRuleParam {
                    field: "task_name".into(),
                    operator: "equals".into(),
                    value: DataValue::Text("alpha".into()),
                }],
            }),
            sort: None,
        };
        assert_eq!(extract_status_filter(&request), None);
    }

    #[test]
    fn apply_jobs_datatable_query_filters_by_quick_search() {
        let mut jobs = vec![
            sample_job("j1", "alpha", JobStatusDto::Queued),
            sample_job("j2", "beta", JobStatusDto::Running),
        ];
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: Some("alpha".into()),
            filter: None,
            sort: None,
        };
        apply_jobs_datatable_query(&mut jobs, &request);
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].job_id, "j1");
    }

    #[test]
    fn apply_runs_datatable_query_status_equals() {
        let mut runs = vec![
            sample_run("r1", "j1", "alpha", RunStatusDto::Success),
            sample_run("r2", "j2", "beta", RunStatusDto::Failed),
        ];
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: None,
            filter: Some(FilterQuery {
                logic: FilterLogicWire::And,
                items: vec![FilterRuleParam {
                    field: "status".into(),
                    operator: "equals".into(),
                    value: DataValue::Text("failed".into()),
                }],
            }),
            sort: None,
        };
        apply_runs_datatable_query(&mut runs, &request);
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].run_id, "r2");
    }

    #[test]
    fn apply_jobs_datatable_query_or_logic_keeps_either_match() {
        let mut jobs = vec![
            sample_job("j1", "alpha", JobStatusDto::Queued),
            sample_job("j2", "beta", JobStatusDto::Running),
            sample_job("j3", "gamma", JobStatusDto::Failed),
        ];
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: None,
            filter: Some(FilterQuery {
                logic: FilterLogicWire::Or,
                items: vec![
                    FilterRuleParam {
                        field: "task_name".into(),
                        operator: "equals".into(),
                        value: DataValue::Text("alpha".into()),
                    },
                    FilterRuleParam {
                        field: "status".into(),
                        operator: "equals".into(),
                        value: DataValue::Text("failed".into()),
                    },
                ],
            }),
            sort: None,
        };
        apply_jobs_datatable_query(&mut jobs, &request);
        assert_eq!(jobs.len(), 2);
        assert!(jobs.iter().any(|j| j.job_id == "j1"));
        assert!(jobs.iter().any(|j| j.job_id == "j3"));
    }

    #[test]
    fn apply_runs_datatable_query_not_equals_status() {
        let mut runs = vec![
            sample_run("r1", "j1", "alpha", RunStatusDto::Success),
            sample_run("r2", "j2", "beta", RunStatusDto::Failed),
        ];
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: None,
            filter: Some(FilterQuery {
                logic: FilterLogicWire::And,
                items: vec![FilterRuleParam {
                    field: "status".into(),
                    operator: "not_equals".into(),
                    value: DataValue::Text("success".into()),
                }],
            }),
            sort: None,
        };
        apply_runs_datatable_query(&mut runs, &request);
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].run_id, "r2");
    }
}
