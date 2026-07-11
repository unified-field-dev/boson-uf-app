//! Dashboard statistics and run trend server functions.

use std::collections::BTreeMap;

use chrono::Timelike;
use leptos::prelude::*;

use super::types::{DashboardChartPoint, DashboardChartSeries, DashboardStats, BOSON_LIST_FETCH_CAP};

#[cfg(feature = "ssr")]
use boson_core::JobStatus;

/// Get dashboard statistics.
#[uf_product_macros::server]
pub async fn get_dashboard_stats() -> Result<DashboardStats, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    let backend = ctx.boson()?;
    let task_count = backend.registry().len() as u32;

    let jobs_queued = backend.count_jobs(Some(JobStatus::Queued)).await as u32;
    let jobs_running = backend.count_jobs(Some(JobStatus::Running)).await as u32;

    let day_ago = chrono::Utc::now() - chrono::Duration::hours(24);
    let runs_today = backend.count_runs_since(day_ago).await as u32;

    Ok(DashboardStats {
        task_count,
        jobs_queued,
        jobs_running,
        runs_today,
    })
}

/// Time-series run outcome counts for the dashboard chart.
///
/// Buckets runs into 4-hour windows (24h range) or daily buckets (7d) by `started_at` in UTC.
/// Empty buckets in the selected range are included so the band axis spans the full window.
/// Series with no non-zero buckets (e.g. Failed when all runs succeeded) are omitted.
/// Uses a bounded backend fetch (0.1.n limitation).
#[uf_product_macros::server]
pub async fn get_run_stats_series(
    range_secs: i64,
) -> Result<Vec<DashboardChartSeries>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    let backend = ctx.boson()?;

    let now = chrono::Utc::now();
    let since = now - chrono::Duration::seconds(range_secs);
    let bucket = run_bucket_granularity(range_secs);

    let runs = backend.list_runs(None, 0, BOSON_LIST_FETCH_CAP).await;

    let mut success_buckets: BTreeMap<chrono::DateTime<chrono::Utc>, u32> = BTreeMap::new();
    let mut failed_buckets: BTreeMap<chrono::DateTime<chrono::Utc>, u32> = BTreeMap::new();

    for run in runs.iter().filter(|r| r.started_at >= since) {
        let bucket_ts = align_run_bucket(run.started_at, bucket);

        match run.status {
            boson_core::RunStatus::Success => {
                *success_buckets.entry(bucket_ts).or_insert(0) += 1;
            }
            boson_core::RunStatus::Failed
            | boson_core::RunStatus::Timeout
            | boson_core::RunStatus::Canceled => {
                *failed_buckets.entry(bucket_ts).or_insert(0) += 1;
            }
            boson_core::RunStatus::Running => {}
        }
    }

    fill_bucket_range(since, now, bucket, &mut success_buckets);
    fill_bucket_range(since, now, bucket, &mut failed_buckets);

    let success_points: Vec<DashboardChartPoint> = success_buckets
        .into_iter()
        .map(|(ts, value)| DashboardChartPoint {
            ts,
            value: value as f64,
        })
        .collect();
    let failed_points: Vec<DashboardChartPoint> = failed_buckets
        .into_iter()
        .map(|(ts, value)| DashboardChartPoint {
            ts,
            value: value as f64,
        })
        .collect();

    let mut series = vec![DashboardChartSeries {
        id: "successful".into(),
        label: "Successful".into(),
        points: success_points,
    }];
    if failed_points.iter().any(|p| p.value > 0.0) {
        series.push(DashboardChartSeries {
            id: "failed".into(),
            label: "Failed".into(),
            points: failed_points,
        });
    }

    Ok(series)
}

/// Bucket width for the dashboard run-outcomes chart.
#[derive(Clone, Copy)]
enum RunBucketGranularity {
    /// 4-hour buckets for the 24h range (6 x-axis labels).
    FourHourly,
    /// Daily buckets for the 7d range.
    Daily,
}

fn run_bucket_granularity(range_secs: i64) -> RunBucketGranularity {
    if range_secs <= 86_400 {
        RunBucketGranularity::FourHourly
    } else {
        RunBucketGranularity::Daily
    }
}

fn align_run_bucket(
    ts: chrono::DateTime<chrono::Utc>,
    bucket: RunBucketGranularity,
) -> chrono::DateTime<chrono::Utc> {
    let naive = ts.naive_utc();
    match bucket {
        RunBucketGranularity::FourHourly => {
            let aligned_hour = (naive.hour() / 4) * 4;
            let hour = naive
                .date()
                .and_hms_opt(aligned_hour, 0, 0)
                .expect("valid 4-hour bucket");
            chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(hour, chrono::Utc)
        }
        RunBucketGranularity::Daily => {
            let day = naive.date().and_hms_opt(0, 0, 0).unwrap();
            chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(day, chrono::Utc)
        }
    }
}

fn fill_bucket_range(
    since: chrono::DateTime<chrono::Utc>,
    now: chrono::DateTime<chrono::Utc>,
    bucket: RunBucketGranularity,
    buckets: &mut BTreeMap<chrono::DateTime<chrono::Utc>, u32>,
) {
    let mut cursor = align_run_bucket(since, bucket);
    let end = align_run_bucket(now, bucket);
    let step = match bucket {
        RunBucketGranularity::FourHourly => chrono::Duration::hours(4),
        RunBucketGranularity::Daily => chrono::Duration::days(1),
    };
    while cursor <= end {
        buckets.entry(cursor).or_insert(0);
        cursor += step;
    }
}
