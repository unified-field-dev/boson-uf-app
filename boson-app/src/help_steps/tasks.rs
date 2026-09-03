//! Spotlight steps for the Tasks catalog (`/boson/tasks`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Tasks page intro: recipe vs ticket.
#[help_spotlight_step(
    route = "/boson/tasks",
    feature_highlight = "boson-tasks-intro",
    title = "Tasks catalog",
    spotlight = "boson-tasks",
    position = "top",
    order = 10
)]
#[component]
pub fn BosonTasksIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-tasks-intro",
        "A Task is a named kind of background work, the recipe. Jobs are the tickets; this page lists the recipes your system knows.",
        Some("If the table is empty, no tasks are registered yet."),
        &[],
    )
}

/// Search and filters on the tasks list.
#[help_spotlight_step(
    route = "/boson/tasks",
    feature_highlight = "boson-tasks-search",
    title = "Find a task",
    spotlight = "boson-tasks-search",
    position = "bottom",
    order = 20
)]
#[component]
pub fn BosonTasksSearchHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-tasks-search",
        "Use search and filters when the catalog is long. Type part of a name to narrow the list; clear the filters to see every task again.",
        None,
        &[],
    )
}

/// Tasks data table columns.
#[help_spotlight_step(
    route = "/boson/tasks",
    feature_highlight = "boson-tasks-table",
    title = "Reading the table",
    spotlight = "boson-tasks-data-table",
    position = "top",
    order = 30
)]
#[component]
pub fn BosonTasksTableHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-tasks-table",
        "Compare these columns before you open or configure a row.",
        None,
        &[
            "Signature: shape of arguments the recipe needs",
            "Defaults: factory routing (pool / priority)",
            "Effective: routing after your overrides",
            "Queued / Runs: how busy this recipe is",
            "Success rate: share of finished attempts that worked",
        ],
    )
}

/// View action on a task row.
#[help_spotlight_step(
    route = "/boson/tasks",
    feature_highlight = "boson-tasks-view",
    title = "View a task",
    spotlight = "boson-tasks-action-view",
    position = "left",
    order = 40
)]
#[component]
pub fn BosonTasksViewHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-tasks-view",
        "View opens this task's summary page: the same routing fields, plus live metrics and links into queue and run history.",
        None,
        &[],
    )
}

/// Configure action on a task row.
#[help_spotlight_step(
    route = "/boson/tasks",
    feature_highlight = "boson-tasks-configure",
    title = "Configure a task",
    spotlight = "boson-tasks-action-configure",
    position = "left",
    order = 50
)]
#[component]
pub fn BosonTasksConfigureHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-tasks-configure",
        "Configure opens settings for where work runs (pool), how urgent new jobs are (priority), and what happens after a failure (retries). Saving needs a verified email on the account.",
        None,
        &[],
    )
}

/// View Queue action on a task row.
#[help_spotlight_step(
    route = "/boson/tasks",
    feature_highlight = "boson-tasks-view-queue",
    title = "View this task's queue",
    spotlight = "boson-tasks-action-queue",
    position = "left",
    order = 60
)]
#[component]
pub fn BosonTasksViewQueueHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-tasks-view-queue",
        "Opens the live waiting line filtered to jobs for this recipe only, waiting or in progress. From there you can cancel a job or open its runs.",
        None,
        &[],
    )
}

/// View Runs action on a task row.
#[help_spotlight_step(
    route = "/boson/tasks",
    feature_highlight = "boson-tasks-view-runs",
    title = "View this task's runs",
    spotlight = "boson-tasks-action-runs",
    position = "left",
    order = 70
)]
#[component]
pub fn BosonTasksViewRunsHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-tasks-view-runs",
        "Opens attempt history for this recipe. Open a run row later for timing and errors.",
        None,
        &[],
    )
}
