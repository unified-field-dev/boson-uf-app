//! Dashboard statistics and run trend server functions.

use leptos::prelude::*;

#[cfg(feature = "ssr")]
use super::types::BOSON_LIST_FETCH_CAP;
use super::types::{DashboardChartSeries, DashboardStats};

#[cfg(feature = "ssr")]
use super::helpers::{require_session, trace_server_result};
#[cfg(feature = "ssr")]
use boson_core::JobStatus;

/// Get dashboard statistics.
#[uf_product_macros::server]
pub async fn get_dashboard_stats() -> Result<DashboardStats, ServerFnError> {
    let result = async {
        let ctx = higgs::Higgs::from_request().await?;
        require_session(&ctx)?;
        let backend = super::helpers::boson_backend()?;
        let backend = backend.as_ref();
        let task_count = u32::try_from(backend.registry().len()).unwrap_or(u32::MAX);

        let jobs_queued =
            u32::try_from(backend.count_jobs(Some(JobStatus::Queued)).await).unwrap_or(u32::MAX);
        let jobs_running =
            u32::try_from(backend.count_jobs(Some(JobStatus::Running)).await).unwrap_or(u32::MAX);

        let day_ago = chrono::Utc::now() - chrono::Duration::hours(24);
        let runs_today = u32::try_from(backend.count_runs_since(day_ago).await).unwrap_or(u32::MAX);

        Ok(boson_backend::dashboard_stats(
            task_count,
            jobs_queued,
            jobs_running,
            runs_today,
        ))
    }
    .await;
    #[cfg(feature = "ssr")]
    trace_server_result("get_dashboard_stats", &result, None, None, None);
    result
}

/// Time-series run outcome counts for the dashboard chart.
///
/// Buckets runs into 4-hour windows (24h range) or daily buckets (7d) by `started_at` in UTC.
/// Empty buckets in the selected range are included so the band axis spans the full window.
/// Series with no non-zero buckets (e.g. Failed when all runs succeeded) are omitted.
/// Uses a bounded backend fetch (0.1.n limitation).
///
/// `range_secs` must be [`boson_backend::RANGE_SECS_24H`] or [`boson_backend::RANGE_SECS_7D`].
#[uf_product_macros::server]
pub async fn get_run_stats_series(
    /// Width of the trailing time window, in seconds, to aggregate run outcomes over.
    range_secs: i64,
) -> Result<Vec<DashboardChartSeries>, ServerFnError> {
    let result = async {
        boson_backend::validate_range_secs(range_secs)
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        let ctx = higgs::Higgs::from_request().await?;
        require_session(&ctx)?;
        let backend = super::helpers::boson_backend()?;
        let backend = backend.as_ref();

        let now = chrono::Utc::now();
        let runs = backend.list_runs(None, 0, BOSON_LIST_FETCH_CAP).await;
        Ok(boson_backend::run_stats_series_from_runs(
            &runs, now, range_secs,
        ))
    }
    .await;
    #[cfg(feature = "ssr")]
    trace_server_result("get_run_stats_series", &result, None, None, None);
    result
}
