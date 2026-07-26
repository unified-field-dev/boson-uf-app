//! Dashboard statistics and run trend server functions.

use leptos::prelude::*;

use super::types::{DashboardChartSeries, DashboardStats, BOSON_LIST_FETCH_CAP};

#[cfg(feature = "ssr")]
use boson_core::JobStatus;

/// Get dashboard statistics.
#[uf_product_macros::server]
pub async fn get_dashboard_stats() -> Result<DashboardStats, ServerFnError> {
    let backend = super::helpers::boson_backend()?;
    let backend = backend.as_ref();
    let task_count = backend.registry().len() as u32;

    let jobs_queued = backend.count_jobs(Some(JobStatus::Queued)).await as u32;
    let jobs_running = backend.count_jobs(Some(JobStatus::Running)).await as u32;

    let day_ago = chrono::Utc::now() - chrono::Duration::hours(24);
    let runs_today = backend.count_runs_since(day_ago).await as u32;

    Ok(boson_backend::dashboard_stats(
        task_count,
        jobs_queued,
        jobs_running,
        runs_today,
    ))
}

/// Time-series run outcome counts for the dashboard chart.
///
/// Buckets runs into 4-hour windows (24h range) or daily buckets (7d) by `started_at` in UTC.
/// Empty buckets in the selected range are included so the band axis spans the full window.
/// Series with no non-zero buckets (e.g. Failed when all runs succeeded) are omitted.
/// Uses a bounded backend fetch (0.1.n limitation).
#[uf_product_macros::server]
pub async fn get_run_stats_series(
    /// Width of the trailing time window, in seconds, to aggregate run outcomes over.
    range_secs: i64,
) -> Result<Vec<DashboardChartSeries>, ServerFnError> {
    let backend = super::helpers::boson_backend()?;
    let backend = backend.as_ref();

    let now = chrono::Utc::now();
    let runs = backend.list_runs(None, 0, BOSON_LIST_FETCH_CAP).await;
    Ok(boson_backend::run_stats_series_from_runs(
        &runs, now, range_secs,
    ))
}
