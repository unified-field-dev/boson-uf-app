//! Spotlight steps for the Boson dashboard (`/boson`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Centered intro: control room metaphor and Task / Job / Run vocabulary.
#[help_spotlight_step(
    route = "/boson",
    feature_highlight = "boson-intro",
    title = "Welcome to Boson",
    order = 10
)]
#[component]
pub fn BosonIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-intro",
        "Boson is the control room for background work. Think of it like a kitchen ticket system: someone places an order (a job), cooks prepare it (workers), and you can see what is waiting, cooking, or done.",
        Some("We will walk the screens one piece at a time."),
        &[
            "Task: the recipe (a kind of background work)",
            "Job: one ticket asking for that recipe",
            "Run: one attempt to finish that ticket",
        ],
    )
}

/// KPI cards: Tasks, Queued, Running, Runs (24h).
#[help_spotlight_step(
    route = "/boson",
    feature_highlight = "boson-dashboard-stats",
    title = "At a glance",
    spotlight = "boson-dashboard-stats",
    position = "bottom",
    order = 20
)]
#[component]
pub fn BosonDashboardStatsHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-dashboard-stats",
        "These four numbers are today's pulse.",
        Some("Come back here when you want a quick health check."),
        &[
            "Tasks: how many recipes exist",
            "Queued: tickets waiting for a worker",
            "Running: tickets being worked on now",
            "Runs (24h): finished attempts in the last day",
        ],
    )
}

/// Run outcomes trend chart.
#[help_spotlight_step(
    route = "/boson",
    feature_highlight = "boson-run-trend",
    title = "How work is finishing",
    spotlight = "boson-dashboard-run-trend",
    position = "top",
    order = 30
)]
#[component]
pub fn BosonRunTrendHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-run-trend",
        "The chart compares successful vs failed finishes. Failed includes failed, timeout, and canceled.",
        Some("Tip: switch 24h / 7d, then open View all runs for the full history list."),
        &[],
    )
}

/// Quick link to Tasks.
#[help_spotlight_step(
    route = "/boson",
    feature_highlight = "boson-ql-tasks",
    title = "Open Tasks",
    spotlight = "boson-ql-tasks",
    position = "top",
    order = 40
)]
#[component]
pub fn BosonQlTasksHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-ql-tasks",
        "Tasks is the catalog of recipes. Open it to browse kinds of work, compare load, and jump into configure or history.",
        Some("You can click this card now, or keep touring and use the left menu later."),
        &[],
    )
}

/// Quick link to Queue.
#[help_spotlight_step(
    route = "/boson",
    feature_highlight = "boson-ql-queue",
    title = "Open Queue",
    spotlight = "boson-ql-queue",
    position = "top",
    order = 50
)]
#[component]
pub fn BosonQlQueueHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-ql-queue",
        "Queue is the live waiting line: tickets that are waiting or already being worked. Open it to watch status or cancel a job.",
        None,
        &[],
    )
}

/// Quick link to Runs.
#[help_spotlight_step(
    route = "/boson",
    feature_highlight = "boson-ql-runs",
    title = "Open Runs",
    spotlight = "boson-ql-runs",
    position = "top",
    order = 60
)]
#[component]
pub fn BosonQlRunsHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-ql-runs",
        "Runs is the history of attempts. Open it to see whether work succeeded, failed, or is still going, and to open one attempt.",
        None,
        &[],
    )
}

/// Tasks overview table on the dashboard.
#[help_spotlight_step(
    route = "/boson",
    feature_highlight = "boson-tasks-overview",
    title = "Your busiest tasks",
    spotlight = "boson-dashboard-tasks-overview",
    position = "top",
    order = 70
)]
#[component]
pub fn BosonTasksOverviewHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-tasks-overview",
        "This table lists a few top tasks with queued tickets, run counts, and success rate.",
        Some("Use it to spot busy or unreliable recipes, then open Tasks for the full catalog."),
        &[
            "Success rate is the share of finished attempts that worked. A dash means nothing has finished yet.",
        ],
    )
}

/// Left navigation destinations.
#[help_spotlight_step(
    route = "/boson",
    feature_highlight = "boson-nav",
    title = "Finding your way",
    spotlight = "boson-nav",
    position = "right",
    order = 80
)]
#[component]
pub fn BosonNavHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-nav",
        "Use the left menu to open Dashboard for a health overview, Tasks for recipes and settings, Queue for live tickets, and Runs for attempt history.",
        Some("Help → Replay restarts this page's tour."),
        &[],
    )
}
