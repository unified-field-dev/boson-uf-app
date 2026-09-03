//! Spotlight steps for task detail (`/boson/tasks/:task_name`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Task identity and routing summary.
#[help_spotlight_step(
    route = "/boson/tasks/:task_name",
    feature_highlight = "boson-task-detail-summary",
    title = "This task",
    spotlight = "boson-task-detail-summary",
    position = "bottom",
    order = 10
)]
#[component]
pub fn BosonTaskDetailSummaryHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-task-detail-summary",
        "You are looking at one recipe. Signature is the argument shape. Defaults are factory routing; Effective is what runs after your overrides.",
        Some("Use Back to return to the Tasks catalog."),
        &[],
    )
}

/// Live metrics for this task.
#[help_spotlight_step(
    route = "/boson/tasks/:task_name",
    feature_highlight = "boson-task-detail-metrics",
    title = "How it is doing",
    spotlight = "boson-task-detail-metrics",
    position = "top",
    order = 20
)]
#[component]
pub fn BosonTaskDetailMetricsHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-task-detail-metrics",
        "Use these numbers to choose Configure, Queue, or Runs.",
        None,
        &[
            "Queued: tickets waiting right now",
            "Runs: attempts recorded for this recipe",
            "Success rate: share of finished attempts that worked (dash if none finished)",
        ],
    )
}

/// Configure button on task detail.
#[help_spotlight_step(
    route = "/boson/tasks/:task_name",
    feature_highlight = "boson-task-detail-configure",
    title = "Configure",
    spotlight = "task-detail-configure",
    position = "top",
    order = 30
)]
#[component]
pub fn BosonTaskDetailConfigureHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-task-detail-configure",
        "Opens the settings page for this recipe: choose the worker pool, default priority, and retry policy, then Save.",
        None,
        &[],
    )
}

/// View Queue button on task detail.
#[help_spotlight_step(
    route = "/boson/tasks/:task_name",
    feature_highlight = "boson-task-detail-view-queue",
    title = "View Queue",
    spotlight = "task-detail-view-queue",
    position = "top",
    order = 40
)]
#[component]
pub fn BosonTaskDetailViewQueueHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-task-detail-view-queue",
        "Opens live jobs for this recipe only. From the queue you can cancel a ticket or click a row to see that job's attempts.",
        None,
        &[],
    )
}

/// View Runs button on task detail.
#[help_spotlight_step(
    route = "/boson/tasks/:task_name",
    feature_highlight = "boson-task-detail-view-runs",
    title = "View Runs",
    spotlight = "task-detail-view-runs",
    position = "top",
    order = 50
)]
#[component]
pub fn BosonTaskDetailViewRunsHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-task-detail-view-runs",
        "Opens past attempts for this recipe. Click a run to read timing and any error detail.",
        None,
        &[],
    )
}
