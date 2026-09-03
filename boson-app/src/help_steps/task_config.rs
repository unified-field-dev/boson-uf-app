//! Spotlight steps for task config (`/boson/tasks/:task_name/config`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Task settings page intro.
#[help_spotlight_step(
    route = "/boson/tasks/:task_name/config",
    feature_highlight = "boson-task-config-intro",
    title = "Task settings",
    spotlight = "boson-task-config",
    position = "top",
    order = 10
)]
#[component]
pub fn BosonTaskConfigIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-task-config-intro",
        "This page changes how future jobs for this recipe are routed and how failures are retried.",
        Some("Nothing is written until you press Save."),
        &[],
    )
}

/// Worker pool field.
#[help_spotlight_step(
    route = "/boson/tasks/:task_name/config",
    feature_highlight = "boson-task-config-pool",
    title = "Worker pool",
    spotlight = "task-config-pool",
    position = "bottom",
    order = 20
)]
#[component]
pub fn BosonTaskConfigPoolHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-task-config-pool",
        "The pool chooses which worker team runs this work. When no custom pools exist, global is the default.",
        Some("Pick the pool that should own this recipe."),
        &[],
    )
}

/// Default priority field.
#[help_spotlight_step(
    route = "/boson/tasks/:task_name/config",
    feature_highlight = "boson-task-config-priority",
    title = "Default priority",
    spotlight = "task-config-priority",
    position = "bottom",
    order = 30
)]
#[component]
pub fn BosonTaskConfigPriorityHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-task-config-priority",
        "When workers are busy, lower numbers run sooner. This value becomes the default for new jobs of this recipe (unless a job sets its own priority).",
        None,
        &[],
    )
}

/// Retry policy card.
#[help_spotlight_step(
    route = "/boson/tasks/:task_name/config",
    feature_highlight = "boson-task-config-retry",
    title = "If a run fails",
    spotlight = "boson-task-config-retry",
    position = "top",
    order = 40
)]
#[component]
pub fn BosonTaskConfigRetryHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-task-config-retry",
        "Like redialing a busy phone line.",
        Some("Set max attempts to 1 to turn retries off."),
        &[
            "Max attempts: total tries including the first",
            "Initial delay: wait before the first retry",
            "Max delay: cap on wait between tries",
            "Backoff: how fast the wait grows",
        ],
    )
}

/// Cancel button discards unsaved edits.
#[help_spotlight_step(
    route = "/boson/tasks/:task_name/config",
    feature_highlight = "boson-task-config-cancel",
    title = "Discard edits",
    spotlight = "task-config-cancel",
    position = "top",
    order = 50
)]
#[component]
pub fn BosonTaskConfigCancelHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-task-config-cancel",
        "Cancel leaves without writing. Unsaved edits are discarded and you return to the task summary.",
        None,
        &[],
    )
}

/// Save button persists overrides.
#[help_spotlight_step(
    route = "/boson/tasks/:task_name/config",
    feature_highlight = "boson-task-config-save",
    title = "Save changes",
    spotlight = "task-config-save",
    position = "top",
    order = 60
)]
#[component]
pub fn BosonTaskConfigSaveHelp() -> impl IntoView {
    help_stack(
        "help-step-boson-task-config-save",
        "Save writes your overrides. Effective routing on the task page updates after a successful save.",
        None,
        &[],
    )
}
