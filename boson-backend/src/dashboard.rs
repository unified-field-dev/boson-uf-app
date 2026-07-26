//! Run-trend bucketing helpers for the Boson dashboard chart.

use std::collections::BTreeMap;

use chrono::{DateTime, Timelike, Utc};

use crate::types::{DashboardChartPoint, DashboardChartSeries};
use boson_core::{Run, RunStatus};

/// Bucket width for the dashboard run-outcomes chart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunBucketGranularity {
    /// 4-hour buckets for the 24h range (6 x-axis labels).
    FourHourly,
    /// Daily buckets for the 7d range.
    Daily,
}

/// Chooses four-hourly buckets for ≤24h ranges and daily buckets otherwise.
#[must_use]
pub const fn run_bucket_granularity(range_secs: i64) -> RunBucketGranularity {
    if range_secs <= 86_400 {
        RunBucketGranularity::FourHourly
    } else {
        RunBucketGranularity::Daily
    }
}

/// Floors a timestamp to the start of its chart bucket.
#[must_use]
pub fn align_run_bucket(ts: DateTime<Utc>, bucket: RunBucketGranularity) -> DateTime<Utc> {
    let naive = ts.naive_utc();
    match bucket {
        RunBucketGranularity::FourHourly => {
            let aligned_hour = (naive.hour() / 4) * 4;
            let hour = naive
                .date()
                .and_hms_opt(aligned_hour, 0, 0)
                .unwrap_or(naive);
            DateTime::<Utc>::from_naive_utc_and_offset(hour, Utc)
        }
        RunBucketGranularity::Daily => {
            let day = naive.date().and_hms_opt(0, 0, 0).unwrap_or(naive);
            DateTime::<Utc>::from_naive_utc_and_offset(day, Utc)
        }
    }
}

/// Ensures every bucket between `since` and `now` exists (zero-filled).
pub fn fill_bucket_range(
    since: DateTime<Utc>,
    now: DateTime<Utc>,
    bucket: RunBucketGranularity,
    buckets: &mut BTreeMap<DateTime<Utc>, u32>,
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

/// Builds successful/failed chart series from a bounded run list.
///
/// Failed series includes Failed / Timeout / Canceled outcomes and is omitted
/// when every bucket is zero. Running attempts are ignored.
#[must_use]
pub fn run_stats_series_from_runs(
    runs: &[Run],
    now: DateTime<Utc>,
    range_secs: i64,
) -> Vec<DashboardChartSeries> {
    let since = now - chrono::Duration::seconds(range_secs);
    let bucket = run_bucket_granularity(range_secs);

    let mut success_buckets: BTreeMap<DateTime<Utc>, u32> = BTreeMap::new();
    let mut failed_buckets: BTreeMap<DateTime<Utc>, u32> = BTreeMap::new();

    for run in runs.iter().filter(|r| r.started_at >= since) {
        let bucket_ts = align_run_bucket(run.started_at, bucket);
        match run.status {
            RunStatus::Success => {
                *success_buckets.entry(bucket_ts).or_insert(0) += 1;
            }
            RunStatus::Failed | RunStatus::Timeout | RunStatus::Canceled => {
                *failed_buckets.entry(bucket_ts).or_insert(0) += 1;
            }
            RunStatus::Running => {}
        }
    }

    fill_bucket_range(since, now, bucket, &mut success_buckets);
    fill_bucket_range(since, now, bucket, &mut failed_buckets);

    let success_points: Vec<DashboardChartPoint> = success_buckets
        .into_iter()
        .map(|(ts, value)| DashboardChartPoint {
            ts,
            value: f64::from(value),
        })
        .collect();
    let failed_points: Vec<DashboardChartPoint> = failed_buckets
        .into_iter()
        .map(|(ts, value)| DashboardChartPoint {
            ts,
            value: f64::from(value),
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
    series
}
