//! Integration contracts for `DataTable` page-query helpers used by
//! `list_jobs_datatable_page` / `list_runs_datatable_page`.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use boson_backend::{
    apply_jobs_datatable_query, apply_runs_datatable_query, extract_status_filter, job_status_key,
    quick_search_text, resolve_job_filter, run_status_key, JobStatusDto, JobSummary, RunStatusDto,
    RunSummary,
};
use orbital_data::DataValue;
use orbital_paging::{FilterLogicWire, FilterQuery, FilterRuleParam, PageRequest};

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
fn jobs_datatable_quick_search_happy_path() {
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
fn jobs_datatable_or_logic_keeps_either_match_happy_path() {
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
fn runs_datatable_status_equals_happy_path() {
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
fn runs_datatable_not_equals_status_happy_path() {
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

#[test]
fn extract_status_filter_reads_equals_happy_path() {
    let request = PageRequest {
        offset: 0,
        limit: 20,
        quick_search: None,
        filter: Some(FilterQuery {
            logic: FilterLogicWire::And,
            items: vec![FilterRuleParam {
                field: "status".into(),
                operator: "equals".into(),
                value: DataValue::Text("Queued".into()),
            }],
        }),
        sort: None,
    };
    assert_eq!(extract_status_filter(&request), Some("queued".into()));
}

#[test]
fn extract_status_filter_non_status_is_none_sad() {
    let request = PageRequest {
        offset: 0,
        limit: 20,
        quick_search: None,
        filter: Some(FilterQuery {
            logic: FilterLogicWire::And,
            items: vec![FilterRuleParam {
                field: "pool".into(),
                operator: "equals".into(),
                value: DataValue::Text("global".into()),
            }],
        }),
        sort: None,
    };
    assert_eq!(extract_status_filter(&request), None);
}

#[test]
fn resolve_job_filter_prefers_scope_happy_path() {
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
fn resolve_job_filter_falls_back_to_filter_happy_path() {
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
fn quick_search_text_rejects_blank_sad() {
    let request = PageRequest {
        offset: 0,
        limit: 20,
        quick_search: Some("   ".into()),
        filter: None,
        sort: None,
    };
    assert_eq!(quick_search_text(&request), None);
}

#[test]
fn status_keys_map_variants_happy_path() {
    assert_eq!(job_status_key(JobStatusDto::Queued), "queued");
    assert_eq!(run_status_key(RunStatusDto::Timeout), "timeout");
}
