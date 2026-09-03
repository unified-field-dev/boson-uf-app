//! Spotlight steps for the Runs list (`/boson/runs`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Runs list intro.
#[help_spotlight_step(
    route = "/boson/runs",
    feature_highlight = "boson-runs-intro",
    title = "Run history",
    spotlight = "boson-runs",
    position = "top",
    order = 10
)]
#[component]
pub fn BosonRunsIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-runs-intro",
        "A Run is one attempt to finish a Job. Retries create new runs with a higher attempt number.",
        Some("If a job filter chip is showing, Clear returns you to the full history."),
        &[],
    )
}

/// Runs table columns.
#[help_spotlight_step(
    route = "/boson/runs",
    feature_highlight = "boson-runs-table",
    title = "Each attempt",
    spotlight = "boson-runs-data-table",
    position = "top",
    order = 20
)]
#[component]
pub fn BosonRunsTableHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-runs-table",
        "Compare attempts side by side in this table.",
        None,
        &[
            "Attempt: 1 is the first try; higher means a retry",
            "Status: succeeded, failed, running, and related states",
            "Started: when the attempt began",
            "Duration: wall time (dash while still running)",
        ],
    )
}

/// Row click opens run detail.
#[help_spotlight_step(
    route = "/boson/runs",
    feature_highlight = "boson-runs-open",
    title = "Open a run",
    spotlight = "boson-runs-data-table",
    position = "bottom",
    order = 30
)]
#[component]
pub fn BosonRunsOpenHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-runs-open",
        "Click a row to open that attempt's detail page: identity, timing, and error text when something failed.",
        None,
        &[],
    )
}
