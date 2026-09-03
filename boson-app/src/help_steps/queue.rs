//! Spotlight steps for the Queue page (`/boson/queue`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Queue page intro: job as ticket.
#[help_spotlight_step(
    route = "/boson/queue",
    feature_highlight = "boson-queue-intro",
    title = "The waiting line",
    spotlight = "boson-queue",
    position = "top",
    order = 10
)]
#[component]
pub fn BosonQueueIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-queue-intro",
        "A Job is one request for a Task, one kitchen ticket. This page shows tickets that are waiting or already being worked.",
        None,
        &[],
    )
}

/// Status filter control.
#[help_spotlight_step(
    route = "/boson/queue",
    feature_highlight = "boson-queue-filter",
    title = "Filter by status",
    spotlight = "queue-status-filter",
    position = "bottom",
    order = 20
)]
#[component]
pub fn BosonQueueFilterHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-queue-filter",
        "Narrow the list to queued, running, or other statuses in plain language. Clear the filter to see active jobs again.",
        None,
        &[],
    )
}

/// Queue data table columns.
#[help_spotlight_step(
    route = "/boson/queue",
    feature_highlight = "boson-queue-table",
    title = "Jobs in flight",
    spotlight = "boson-queue-data-table",
    position = "top",
    order = 30
)]
#[component]
pub fn BosonQueueTableHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-queue-table",
        "Each row is a ticket: which job, which recipe, status badge, pool, priority, and when it was enqueued. Scan for stuck or high-priority work.",
        None,
        &[],
    )
}

/// Row click opens that job's runs.
#[help_spotlight_step(
    route = "/boson/queue",
    feature_highlight = "boson-queue-open-runs",
    title = "Open a job's runs",
    spotlight = "boson-queue-data-table",
    position = "bottom",
    order = 40
)]
#[component]
pub fn BosonQueueOpenRunsHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-queue-open-runs",
        "Click a row to open Runs filtered to that job. That is how you follow every attempt on one ticket.",
        None,
        &[],
    )
}

/// Cancel action on a queue row.
#[help_spotlight_step(
    route = "/boson/queue",
    feature_highlight = "boson-queue-cancel",
    title = "Cancel a job",
    spotlight = "boson-queue-cancel-hint",
    position = "left",
    order = 50
)]
#[component]
pub fn BosonQueueCancelHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-queue-cancel",
        "Cancel stops a waiting or running ticket when your account is allowed. If you are not allowed, the app shows a clear error.",
        None,
        &[],
    )
}
