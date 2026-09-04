//! Integration contracts for dashboard KPI / run-trend helpers.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use boson_backend::{
    align_run_bucket, dashboard_stats, default_gluon_pool_rows, run_bucket_granularity,
    run_stats_series_from_runs, RunBucketGranularity,
};
use boson_core::{Run, RunStatus};
use chrono::{TimeZone, Timelike, Utc};

fn sample_run(id: &str, status: RunStatus, started_at: chrono::DateTime<Utc>) -> Run {
    Run {
        run_id: id.into(),
        job_id: "j1".into(),
        task_name: "alpha".into(),
        attempt: 1,
        status,
        started_at,
        finished_at: Some(started_at),
        duration_ms: Some(1),
        error_message: None,
    }
}

#[test]
fn dashboard_stats_aggregates_counts_happy_path() {
    let stats = dashboard_stats(2, 5, 1, 12);
    assert_eq!(stats.task_count, 2);
    assert_eq!(stats.jobs_queued, 5);
    assert_eq!(stats.jobs_running, 1);
    assert_eq!(stats.runs_today, 12);
}

#[test]
fn run_stats_series_24h_includes_success_and_failed_happy_path() {
    let now = Utc.with_ymd_and_hms(2026, 1, 2, 12, 0, 0).unwrap();
    let runs = vec![
        sample_run("r1", RunStatus::Success, now - chrono::Duration::hours(2)),
        sample_run("r2", RunStatus::Failed, now - chrono::Duration::hours(3)),
        sample_run("r3", RunStatus::Timeout, now - chrono::Duration::hours(4)),
    ];
    let series = run_stats_series_from_runs(&runs, now, 86_400);
    assert_eq!(series.len(), 2);
    assert_eq!(series[0].id, "successful");
    assert_eq!(series[1].id, "failed");
    assert!(series[0].points.iter().any(|p| p.value > 0.0));
    assert!(series[1].points.iter().any(|p| p.value > 0.0));
}

#[test]
fn run_stats_series_all_outside_window_zero_success_sad() {
    let now = Utc.with_ymd_and_hms(2026, 1, 2, 12, 0, 0).unwrap();
    let runs = vec![sample_run(
        "r1",
        RunStatus::Success,
        now - chrono::Duration::hours(48),
    )];
    let series = run_stats_series_from_runs(&runs, now, 86_400);
    assert_eq!(series.len(), 1);
    assert!(series[0].points.iter().all(|p| p.value == 0.0));
}

#[test]
fn run_bucket_granularity_and_align_happy_path() {
    assert_eq!(
        run_bucket_granularity(86_400),
        RunBucketGranularity::FourHourly
    );
    let ts = Utc.with_ymd_and_hms(2026, 1, 1, 10, 45, 0).unwrap();
    let aligned = align_run_bucket(ts, RunBucketGranularity::FourHourly);
    assert_eq!(aligned.hour(), 8);
    let daily = align_run_bucket(ts, RunBucketGranularity::Daily);
    assert_eq!(daily.hour(), 0);
}

#[test]
fn default_gluon_pool_rows_shape_happy_path() {
    let rows = default_gluon_pool_rows();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].id, "global");
    assert_ne!(rows[0].label, "");
}
