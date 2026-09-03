//! Spotlight steps for run detail (`/boson/runs/:id`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Run identity fields.
#[help_spotlight_step(
    route = "/boson/runs/:id",
    feature_highlight = "boson-run-detail-info",
    title = "This attempt",
    spotlight = "boson-run-detail-info",
    position = "bottom",
    order = 10
)]
#[component]
pub fn BosonRunDetailInfoHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-run-detail-info",
        "Run id, task name, status, and attempt index tell you which try you are reading. Use Back for the runs list.",
        None,
        &[],
    )
}

/// Related job link.
#[help_spotlight_step(
    route = "/boson/runs/:id",
    feature_highlight = "boson-run-detail-job",
    title = "Related job",
    spotlight = "boson-run-detail-job",
    position = "top",
    order = 20
)]
#[component]
pub fn BosonRunDetailJobHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-run-detail-job",
        "The job id ties this attempt to its ticket. Click it to open Runs filtered to that job so you can compare every try.",
        None,
        &[],
    )
}

/// Timing and status fields.
#[help_spotlight_step(
    route = "/boson/runs/:id",
    feature_highlight = "boson-run-detail-timing",
    title = "Timing and status",
    spotlight = "boson-run-detail-timing",
    position = "top",
    order = 30
)]
#[component]
pub fn BosonRunDetailTimingHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-run-detail-timing",
        "Started, finished, and duration describe this try. While status is Running, duration updates live.",
        None,
        &[],
    )
}

/// Error detail region.
#[help_spotlight_step(
    route = "/boson/runs/:id",
    feature_highlight = "boson-run-detail-error",
    title = "When something fails",
    spotlight = "boson-run-detail-error",
    position = "top",
    order = 40
)]
#[component]
pub fn BosonRunDetailErrorHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-run-detail-error",
        "If this attempt failed, the error detail appears here. When the run succeeded, this region stays empty; we still highlight it so you know where to look next time.",
        None,
        &[],
    )
}
