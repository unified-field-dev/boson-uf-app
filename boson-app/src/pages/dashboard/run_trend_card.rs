use leptos::prelude::*;
use orbital::components::{
    Button, Caption1, Card, Flex, FlexGap, FlexWrap, MessageBar, MessageBarIntent,
    Skeleton, SkeletonItem, SpacingSize,
};
use orbital::primitives::*;

use super::charts::{line_chart_from_series, run_outcome_series_is_empty};
use crate::components::{run_outcomes_chart_help, BosonCardContent, BosonHelpCardHeader};
use crate::server::get_run_stats_series;

const RANGE_24H: i64 = 86_400;
const RANGE_7D: i64 = 604_800;

#[component]
fn RunTrendChartSkeleton(
    /// Additional CSS class(es) to apply.
    #[prop(into)] class: String,
) -> impl IntoView {
    view! {
        <Skeleton>
            <SkeletonItem class=class />
        </Skeleton>
    }
}

#[component]
pub fn BosonRunTrendCard() -> impl IntoView {
    let range_secs = RwSignal::new(RANGE_24H);
    let res = Resource::new(
        move || range_secs.get(),
        |secs| async move { get_run_stats_series(secs).await },
    );

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .ChartSkeleton { width: 100%; height: 280px; }
    };
    let chart_skeleton_class = StoredValue::new(class_names.chart_skeleton.to_string());

    view! {
        <style>{style_sheet}</style>
        <Card>
            <BosonHelpCardHeader
                title="Run outcomes"
                info=run_outcomes_chart_help()
            />
            <BosonCardContent>
                <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
                    <Flex gap=FlexGap::Small wrap=FlexWrap::Wrap>
                        <div data-testid="dashboard-run-trend-range-24h">
                            <Button
                                appearance=Signal::derive(move || {
                                    if range_secs.get() == RANGE_24H {
                                        ButtonAppearance::Primary
                                    } else {
                                        ButtonAppearance::Secondary
                                    }
                                })
                                on:click=move |_| range_secs.set(RANGE_24H)
                            >
                                "24h"
                            </Button>
                        </div>
                        <div data-testid="dashboard-run-trend-range-7d">
                            <Button
                                appearance=Signal::derive(move || {
                                    if range_secs.get() == RANGE_7D {
                                        ButtonAppearance::Primary
                                    } else {
                                        ButtonAppearance::Secondary
                                    }
                                })
                                on:click=move |_| range_secs.set(RANGE_7D)
                            >
                                "7d"
                            </Button>
                        </div>
                    </Flex>

                    <div data-testid="dashboard-run-trend-chart">
                        <Show
                            when=move || res.get().is_some()
                            fallback=move || view! {
                                <RunTrendChartSkeleton class=chart_skeleton_class.with_value(|c| c.clone()) />
                            }
                        >
                            <Transition fallback=move || view! {
                                <RunTrendChartSkeleton class=chart_skeleton_class.with_value(|c| c.clone()) />
                            }>
                                {move || res.get().map(|r| match r {
                                    Ok(series) if run_outcome_series_is_empty(&series) => view! {
                                        <MessageBar intent=MessageBarIntent::Info>
                                            "No runs in this time range."
                                        </MessageBar>
                                    }.into_any(),
                                    Ok(series) => {
                                        let use_daily = range_secs.get_untracked() > RANGE_24H;
                                        view! {
                                            {line_chart_from_series(&series, 280.0, use_daily)}
                                        }.into_any()
                                    }
                                    Err(e) => view! {
                                        <MessageBar intent=MessageBarIntent::Error>
                                            "Failed to load chart: " {e.to_string()}
                                        </MessageBar>
                                    }.into_any(),
                                })}
                            </Transition>
                        </Show>
                    </div>

                    <div data-testid="dashboard-run-trend-view-all">
                        <Link href=crate::paths::RUNS>
                            <Caption1>"View all runs →"</Caption1>
                        </Link>
                    </div>
                </Flex>
            </BosonCardContent>
        </Card>
    }
}
