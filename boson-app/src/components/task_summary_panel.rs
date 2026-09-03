use leptos::prelude::*;
use orbital::components::{Body1, Caption2, SpacingSize, Subtitle2, Text, TextTag};
use orbital::primitives::{Flex, FlexAlign, FlexWrap, InfoLabel, InfoLabelInfo};

use crate::components::help::{defaults_help, effective_help, signature_help};
use crate::server::TaskSummary;

fn effective_config_label(task: &TaskSummary) -> String {
    if task.effective_priority != task.default_priority || task.effective_pool != task.default_pool
    {
        format!(
            "pool=\"{}\", priority={} (UI override)",
            task.effective_pool, task.effective_priority
        )
    } else {
        format!(
            "pool=\"{}\", priority={} (using defaults)",
            task.effective_pool, task.effective_priority
        )
    }
}

fn success_rate_label(task: &TaskSummary) -> String {
    task.success_rate_pct
        .map_or_else(|| "-".to_string(), |r| format!("{r:.0}%"))
}

/// Shared task metadata block for card and detail views.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn TaskSummaryPanel(
    /// Task to display.
    task: TaskSummary,
    /// Whether to show title.
    #[prop(default = false)]
    show_title: bool,
    /// Child content rendered inside the component.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .MetaSecondary {
            color: var(--orb-color-text-tertiary);
        }
        .Actions { flex-wrap: wrap; }
    };

    let name = task.name.clone();
    let signature = task.signature_json.clone();
    let default_pool = task.default_pool.clone();
    let default_priority = task.default_priority;
    let effective_str = effective_config_label(&task);
    let success_str = success_rate_label(&task);
    let jobs_queued = task.jobs_queued;
    let runs_total = task.runs_total;

    view! {
        <style>{style_sheet}</style>
        <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
            {show_title.then(|| view! { <Subtitle2 block=true>{name.clone()}</Subtitle2> })}
            <div id="boson-task-detail-summary">
                <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                    <Flex align=FlexAlign::Center gap=SpacingSize::Size40.flex_gap() wrap=FlexWrap::Wrap>
                        <InfoLabel>
                            <Body1>"Signature"</Body1>
                            <InfoLabelInfo slot>
                                {signature_help()}
                            </InfoLabelInfo>
                        </InfoLabel>
                        <Text tag=TextTag::Code>{signature}</Text>
                    </Flex>
                    <Caption2 block=true class=class_names.meta_secondary>
                        <InfoLabel>
                            "Defaults: "
                            <InfoLabelInfo slot>
                                {defaults_help()}
                            </InfoLabelInfo>
                        </InfoLabel>
                        "pool=\"" {default_pool} "\", priority=" {default_priority}
                    </Caption2>
                    <Caption2 block=true class=class_names.meta_secondary>
                        <InfoLabel>
                            "Effective: "
                            <InfoLabelInfo slot>
                                {effective_help()}
                            </InfoLabelInfo>
                        </InfoLabel>
                        {effective_str}
                    </Caption2>
                </Flex>
            </div>
            <div id="boson-task-detail-metrics">
                <Caption2 block=true class=class_names.meta_secondary>
                    "Jobs queued: " {jobs_queued}
                    "  |  Runs: " {runs_total}
                    "  |  Success rate: " {success_str}
                </Caption2>
            </div>
            {children.map(|c| view! {
                <Flex gap=SpacingSize::Size80.flex_gap() class=class_names.actions>
                    {c()}
                </Flex>
            })}
        </Flex>
    }
}
