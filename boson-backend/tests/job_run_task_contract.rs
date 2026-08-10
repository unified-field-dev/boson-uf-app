//! Integration contracts for job/run/task helpers backing
//! `get_tasks` / `get_task` / `list_jobs_*` / `get_run` / `cancel_job`.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use boson_backend::{
    aggregate_task_stats, apply_task_config_update, filter_tasks_by_query, find_job_by_id,
    find_run_by_id, find_task_by_name, job_to_summary, parse_job_status_filter, run_to_summary,
    sort_tasks_by_name, task_config_to_dto, task_summary_from_parts, validate_job_id,
    validate_run_id, validate_task_name, JobStatusDto, RunStatusDto, TaskStatsAgg, TaskSummary,
    UpdateTaskConfigRequest,
};
use boson_core::{Job, JobStatus, RetryPolicy, Run, RunStatus, TaskConfig};
use chrono::{TimeZone, Utc};

fn sample_task(name: &str) -> TaskSummary {
    TaskSummary {
        name: name.into(),
        signature_json: r#"{"args":[]}"#.into(),
        default_priority: 0,
        default_pool: "global".into(),
        effective_priority: 0,
        effective_pool: "global".into(),
        jobs_queued: 0,
        runs_total: 0,
        success_rate_pct: None,
    }
}

fn sample_job(id: &str, task: &str, status: JobStatus) -> Job {
    Job {
        job_id: id.into(),
        task_name: task.into(),
        actor_json: serde_json::json!({}),
        params_json: serde_json::json!({}),
        priority: 1,
        pool: "global".into(),
        status,
        idempotency_key: None,
        created_at: Utc::now(),
        signature_hash: 0,
        attempt: 1,
    }
}

fn sample_run(id: &str, job: &str, task: &str, status: RunStatus) -> Run {
    let now = Utc::now();
    Run {
        run_id: id.into(),
        job_id: job.into(),
        task_name: task.into(),
        attempt: 1,
        status,
        started_at: now,
        finished_at: Some(now),
        duration_ms: Some(5),
        error_message: None,
    }
}

#[test]
fn get_tasks_list_sorted_and_named_happy_path() {
    let mut tasks = vec![sample_task("zeta.task"), sample_task("alpha.task")];
    sort_tasks_by_name(&mut tasks);
    assert_eq!(tasks[0].name, "alpha.task");
    assert_eq!(tasks[1].name, "zeta.task");
    for t in &tasks {
        assert_ne!(t.name.trim(), "");
        assert_ne!(t.signature_json, "");
    }
}

#[test]
fn get_task_detail_matches_list_entry_happy_path() {
    let tasks = vec![sample_task("orders"), sample_task("payments")];
    let detail = find_task_by_name(&tasks, "orders").expect("listed task must resolve");
    assert_eq!(detail.name, "orders");
    assert_eq!(detail.default_pool, "global");
}

#[test]
fn get_task_unknown_name_is_none_sad() {
    let tasks = vec![sample_task("orders")];
    assert!(find_task_by_name(&tasks, "__boson_uf_app_no_such_task__").is_none());
}

#[test]
fn get_run_detail_matches_list_entry_happy_path() {
    let runs = vec![
        run_to_summary(sample_run("r1", "j1", "orders", RunStatus::Success)),
        run_to_summary(sample_run("r2", "j2", "payments", RunStatus::Failed)),
    ];
    let detail = find_run_by_id(&runs, "r2").expect("listed run must resolve");
    assert_eq!(detail.run_id, "r2");
    assert_eq!(detail.task_name, "payments");
    assert_eq!(detail.status, RunStatusDto::Failed);
}

#[test]
fn get_run_unknown_id_is_none_sad() {
    let runs = vec![run_to_summary(sample_run(
        "r1",
        "j1",
        "orders",
        RunStatus::Success,
    ))];
    assert!(find_run_by_id(&runs, "__boson_uf_app_no_such_run__").is_none());
}

#[test]
fn cancel_job_list_entry_resolves_happy_path() {
    let jobs = vec![
        job_to_summary(sample_job("j1", "orders", JobStatus::Queued)),
        job_to_summary(sample_job("j2", "payments", JobStatus::Running)),
    ];
    let detail = find_job_by_id(&jobs, "j1").expect("listed job must resolve");
    assert_eq!(detail.status, JobStatusDto::Queued);
    assert_eq!(detail.task_name, "orders");
}

#[test]
fn cancel_job_unknown_id_is_none_sad() {
    let jobs = vec![job_to_summary(sample_job(
        "j1",
        "orders",
        JobStatus::Queued,
    ))];
    assert!(find_job_by_id(&jobs, "__boson_uf_app_no_such_job__").is_none());
}

#[test]
fn tasks_page_filters_by_query_happy_path() {
    let mut tasks = vec![
        sample_task("alpha"),
        sample_task("beta"),
        sample_task("gamma"),
    ];
    filter_tasks_by_query(&mut tasks, Some("bet"));
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].name, "beta");
}

#[test]
fn tasks_page_filters_unknown_query_empty_sad() {
    let mut tasks = vec![sample_task("alpha")];
    filter_tasks_by_query(&mut tasks, Some("__no_match__"));
    assert_eq!(tasks.len(), 0);
}

#[test]
fn list_jobs_status_filter_parses_known_happy_path() {
    assert_eq!(parse_job_status_filter("running"), Some(JobStatus::Running));
    assert_eq!(
        parse_job_status_filter("canceled"),
        Some(JobStatus::Canceled)
    );
}

#[test]
fn list_jobs_status_filter_unknown_is_none_sad() {
    assert_eq!(parse_job_status_filter("QUEUED"), None);
    assert_eq!(parse_job_status_filter("bogus"), None);
}

#[test]
fn get_tasks_aggregates_stats_happy_path() {
    let jobs = vec![
        sample_job("j1", "alpha", JobStatus::Queued),
        sample_job("j2", "alpha", JobStatus::Queued),
    ];
    let runs = vec![
        sample_run("r1", "j1", "alpha", RunStatus::Success),
        sample_run("r2", "j1", "alpha", RunStatus::Failed),
    ];
    let stats = aggregate_task_stats(&jobs, &runs);
    let agg = stats.get("alpha").expect("alpha stats");
    assert_eq!(
        *agg,
        TaskStatsAgg {
            jobs_queued: 2,
            runs_total: 2,
            success_count: 1,
        }
    );
    let config = TaskConfig::default_for("alpha");
    let summary = task_summary_from_parts("alpha", "{}", 0, "global", &config, *agg);
    assert_eq!(summary.jobs_queued, 2);
    assert_eq!(summary.success_rate_pct, Some(50.0));
}

#[test]
fn update_task_config_merges_partial_happy_path() {
    let mut config = TaskConfig::default_for("alpha");
    let original_pool = config.pool.clone();
    let req = UpdateTaskConfigRequest {
        priority: Some(11),
        pool: None,
        retry_policy: Some(boson_backend::RetryPolicyDto {
            max_attempts: 4,
            base_delay_ms: 200,
            backoff_multiplier: 2.0,
            max_delay_ms: 10_000,
        }),
    };
    let now = Utc.with_ymd_and_hms(2026, 7, 25, 12, 0, 0).unwrap();
    apply_task_config_update(&mut config, &req, now);
    let dto = task_config_to_dto(&config);
    assert_eq!(dto.priority, 11);
    assert_eq!(dto.pool, original_pool);
    assert_eq!(dto.retry_policy.max_attempts, 4);
    assert_eq!(dto.updated_at, now.to_rfc3339());
}

#[test]
fn validate_task_name_accepts_table_happy_path() {
    validate_task_name("orders.sync").unwrap();
}

#[test]
fn validate_task_name_rejects_blank_sad() {
    let err = validate_task_name("").expect_err("blank");
    assert!(err.contains("required"), "{err}");
}

#[test]
fn validate_job_id_accepts_id_happy_path() {
    validate_job_id("job-abc").unwrap();
}

#[test]
fn validate_job_id_rejects_blank_sad() {
    let err = validate_job_id(" ").expect_err("blank");
    assert!(err.contains("required"), "{err}");
}

#[test]
fn validate_run_id_accepts_id_happy_path() {
    validate_run_id("run-abc").unwrap();
}

#[test]
fn validate_run_id_rejects_blank_sad() {
    let err = validate_run_id("").expect_err("blank");
    assert!(err.contains("required"), "{err}");
}

#[test]
fn retry_policy_roundtrip_fields_happy_path() {
    let policy = RetryPolicy {
        max_attempts: 5,
        base_delay_ms: 100,
        backoff_multiplier: 2.0,
        max_delay_ms: 30_000,
    };
    let dto = boson_backend::retry_policy_to_dto(&policy);
    let back = boson_backend::retry_policy_from_dto(&dto);
    assert_eq!(back.max_attempts, 5);
    assert_eq!(back.base_delay_ms, 100);
    assert!((back.backoff_multiplier - 2.0).abs() < f64::EPSILON);
    assert_eq!(back.max_delay_ms, 30_000);
}
