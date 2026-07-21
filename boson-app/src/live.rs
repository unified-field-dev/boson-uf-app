//! Live-update hooks for Boson job and run lifecycle events.

use leptos::prelude::*;
use serde_json::Value;

/// Reserved for the future broadcast WS wiring (see `BosonJobsLiveSource`).
#[allow(dead_code)]
pub const BOSON_JOBS_WS_PATH: &str = "/ws/boson-jobs";
/// Reserved for the future per-job WS wiring (see `BosonJobRunLiveSource`).
#[allow(dead_code)]
pub const BOSON_JOB_RUN_WS_PREFIX: &str = "/ws/boson-job";

/// Reactive handle to a broadcast job-update subscription.
#[derive(Clone, Copy)]
pub struct BosonJobsSubscription {
    pub trigger: RwSignal<u64>,
    pub latest_event: RwSignal<Option<Value>>,
}

/// Reactive handle to a per-job run-update subscription.
#[derive(Clone, Copy)]
pub struct BosonJobRunSubscription {
    pub trigger: RwSignal<u64>,
    pub latest_event: RwSignal<Option<Value>>,
}

/// Returns true when the event payload targets the given run.
pub fn boson_run_event_matches_run(event: &Value, run_id: &str) -> bool {
    event
        .get("run_id")
        .and_then(|v| v.as_str())
        .is_some_and(|id| id == run_id)
}

/// Returns true for job status transitions (not future streaming kinds).
pub fn boson_job_event_is_status(event: &Value) -> bool {
    event
        .get("kind")
        .and_then(|v| v.as_str())
        .is_some_and(|kind| kind == "status")
}

/// Returns true for run status transitions (not future streaming kinds).
pub fn boson_run_event_is_status(event: &Value) -> bool {
    event
        .get("kind")
        .and_then(|v| v.as_str())
        .is_some_and(|kind| kind == "status")
}

/// Placeholder live source — Photon Axum wiring omitted from this export.
#[component]
pub fn BosonJobsLiveSource(
    /// Two-way signal holding the trigger element/state.
    trigger: RwSignal<u64>,
    /// Two-way signal holding the latest event.
    latest_event: RwSignal<Option<Value>>,
) -> impl IntoView {
    let _ = (trigger, latest_event);
    view! {}
}

/// Placeholder per-job live source — polling refresh is used instead.
#[component]
pub fn BosonJobRunLiveSource(
    /// Reactive signal for the job ID.
    job_id: Signal<Option<String>>,
    /// Two-way signal holding the trigger element/state.
    trigger: RwSignal<u64>,
    /// Two-way signal holding the latest event.
    latest_event: RwSignal<Option<Value>>,
) -> impl IntoView {
    let _ = (job_id, trigger, latest_event);
    view! {}
}

/// Interval for dashboard KPI polling (no broadcast WS). Only read when the
/// `hydrate` feature is enabled.
#[cfg_attr(not(feature = "hydrate"), allow(dead_code))]
pub const BOSON_POLL_INTERVAL_MS: u64 = 20_000;

/// Bump a tick on an interval for resource refresh (client only).
#[cfg(feature = "hydrate")]
pub fn use_boson_poll_tick() -> RwSignal<u32> {
    let tick = RwSignal::new(0u32);
    leptos_use::use_interval_fn(
        move || {
            tick.update(|n| *n += 1);
        },
        BOSON_POLL_INTERVAL_MS,
    );
    tick
}

/// SSR stub — polling runs in the browser bundle only.
#[cfg(not(feature = "hydrate"))]
pub fn use_boson_poll_tick() -> RwSignal<u32> {
    RwSignal::new(0u32)
}

/// Shared subscription signals for the queue broadcast live source.
pub fn boson_jobs_subscription() -> BosonJobsSubscription {
    BosonJobsSubscription {
        trigger: RwSignal::new(0),
        latest_event: RwSignal::new(None),
    }
}

/// Shared subscription signals for a job-scoped live source.
pub fn boson_job_run_subscription() -> BosonJobRunSubscription {
    BosonJobRunSubscription {
        trigger: RwSignal::new(0),
        latest_event: RwSignal::new(None),
    }
}
