//! Plain-language help copy for Boson operator UI.

use leptos::prelude::*;
use orbital::components::Caption1;

fn caption(text: &'static str) -> AnyView {
    view! { <Caption1>{text}</Caption1> }.into_any()
}

pub fn signature_help() -> AnyView {
    caption(
        "JSON shape of arguments registered with #[boson::task]. Used to match payloads when jobs are enqueued.",
    )
}

pub fn defaults_help() -> AnyView {
    caption(
        "Pool and priority from task registration before any per-task UI override is applied.",
    )
}

pub fn effective_help() -> AnyView {
    caption(
        "Resolved routing after per-task configuration. Differs from defaults when you override pool or priority on the config page.",
    )
}

pub fn runs_24h_help() -> AnyView {
    caption(
        "Count of runs whose started_at timestamp falls within the last 24 hours, using the server UTC clock.",
    )
}

pub fn run_outcomes_chart_help() -> AnyView {
    caption(
        "Successful and failed runs bucketed in UTC (hourly for 24h, daily for 7d). \
         Failed includes failed, timeout, and canceled runs. \
         Data is sampled from the most recent run history (bounded fetch). \
         The Runs (24h) stat is a rolling total; this chart shows per-bucket counts for the selected range.",
    )
}

pub fn tasks_overview_help() -> AnyView {
    caption(
        "Top five tasks from the task index ordering. This is not a live activity feed sorted by recent execution.",
    )
}

pub fn basic_config_help() -> AnyView {
    caption(
        "Routes background work to a Gluon virtual pool and sets default priority. Lower priority numbers run sooner.",
    )
}

pub fn pool_field_help() -> AnyView {
    caption(
        "Virtual pool selects which worker capacity runs this task. Default is \"global\" when no Gluon pools are configured.",
    )
}

pub fn retry_policy_help() -> AnyView {
    caption(
        "Controls re-attempts after failure: maximum attempts, initial delay, exponential backoff multiplier, and cap on wait time between retries.",
    )
}

pub fn backoff_multiplier_help() -> AnyView {
    caption(
        "Multiplier applied to the delay after each failed attempt. For example, 2 doubles the wait each retry until max delay is reached.",
    )
}

pub fn max_delay_help() -> AnyView {
    caption(
        "Upper bound on milliseconds to wait between retry attempts, even when backoff would exceed this value.",
    )
}

pub fn pool_help() -> AnyView {
    caption(
        "Virtual pool routes this job to Gluon worker capacity. \"global\" is the default when no pool override is configured.",
    )
}

pub fn priority_help() -> AnyView {
    caption(
        "Job priority within the pool. Lower numbers run sooner when workers are contended.",
    )
}

pub const fn max_attempts_hint() -> &'static str {
    "Total tries including the first run. Set to 1 to disable retries."
}

pub const fn initial_delay_hint() -> &'static str {
    "Milliseconds to wait before the first retry after a failure."
}

pub fn attempt_help() -> AnyView {
    caption(
        "One-based retry index for this run within the job. Attempt 1 is the first try; higher values indicate retries.",
    )
}

pub fn duration_help() -> AnyView {
    caption(
        "Wall-clock time in milliseconds from start to finish. Running jobs show \"-\" until they complete.",
    )
}

pub fn success_rate_help() -> AnyView {
    caption(
        "Percentage of completed runs that succeeded for this task. \"-\" when no runs have finished yet.",
    )
}
