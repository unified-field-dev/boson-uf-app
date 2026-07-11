//! Standalone export stub for the template Photon WebSocket route.
//!
//! Photon Axum wiring is intentionally absent from this repository.

use axum::Router;

pub const BOSON_JOBS_WS_PATH: &str = "/ws/boson-jobs";
pub const BOSON_JOB_RUN_WS_PREFIX: &str = "/ws/boson-job";

/// Preserve the host integration point while omitting Photon Axum wiring.
pub fn merge_routes<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    router
}
