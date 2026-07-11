mod charts;
mod quick_links;
mod recent_tasks_table;
mod run_trend_card;
mod stats_grid;

pub use quick_links::QuickLinks;
pub use recent_tasks_table::RecentTasksTable;
pub use run_trend_card::BosonRunTrendCard;
pub use stats_grid::DashboardStatsGrid;

use leptos::prelude::*;
use orbital::components::{ContentContainer, SpacingSize, Subtitle2, Title3};
use orbital::primitives::*;

use crate::live::use_boson_poll_tick;
use crate::server::{get_dashboard_stats, get_tasks};

#[component]
pub fn BosonRootPage() -> impl IntoView {
    let poll_tick = use_boson_poll_tick();
    let stats_res = Resource::new(|| (), |_| async move { get_dashboard_stats().await });
    let tasks_res = Resource::new(|| (), |_| async move { get_tasks().await });

    Effect::new(move |_| {
        if poll_tick.get() > 0 {
            stats_res.refetch();
        }
    });

    view! {
        <ContentContainer data_testid="boson-dashboard">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                    <Title3>"Boson Dashboard"</Title3>
                    <Subtitle2>"Background work management"</Subtitle2>
                </Flex>

                <DashboardStatsGrid stats_res=stats_res />

                <BosonRunTrendCard />

                <QuickLinks />

                <RecentTasksTable tasks_res=tasks_res />
            </Flex>
        </ContentContainer>
    }
}
