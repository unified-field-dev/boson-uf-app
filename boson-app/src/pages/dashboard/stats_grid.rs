use leptos::prelude::*;
use orbital::components::{
    Caption1, Skeleton, SkeletonItem, SkeletonItemSize, SpacingSize, StatCard,
};
use orbital::primitives::{
    Card, Flex, FlexAlign, FlexWrap, Icon, InfoLabel, InfoLabelInfo, MessageBar, MessageBarIntent,
};

use crate::components::{boson_kpi_enter_motion, runs_24h_help, BosonHelpStatCard};
use crate::server::DashboardStats;
use orbital_motion::{MotionDuration, OrbitalPresenceGroup, OrbitalPresenceGroupItem};

/// Stat card shell with a skeleton placeholder for the value only.
#[component]
fn DashboardStatCardSkeleton(
    /// Label text.
    label: &'static str,
    /// Icon to display.
    icon: icondata_core::Icon,
) -> impl IntoView {
    let value_skeleton = Signal::from(SkeletonItemSize::S32);
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Card { min-width: 140px; flex: 1; }
        .Label { color: var(--orb-color-text-tertiary); }
        .ValueSkeleton { width: 4rem; }
    };

    view! {
        <style>{style_sheet}</style>
        <Card class=class_names.card>
            <Flex vertical=true gap=SpacingSize::Size80.flex_gap() padding=SpacingSize::Size160.inset()>
                <Flex align=FlexAlign::Center gap=SpacingSize::Size80.flex_gap()>
                    <Icon icon=icon />
                    <Caption1 class=class_names.label>{label}</Caption1>
                </Flex>
                <Skeleton>
                    <SkeletonItem class=class_names.value_skeleton size=value_skeleton />
                </Skeleton>
            </Flex>
        </Card>
    }
}

/// Help stat card shell — label and info affordance stay visible; value skeleton only.
#[component]
fn DashboardHelpStatCardSkeleton() -> impl IntoView {
    let value_skeleton = Signal::from(SkeletonItemSize::S32);
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Card { min-width: 140px; flex: 1; }
        .Label { color: var(--orb-color-text-tertiary); }
        .ValueSkeleton { width: 4rem; }
    };

    view! {
        <style>{style_sheet}</style>
        <Card class=class_names.card>
            <Flex vertical=true gap=SpacingSize::Size80.flex_gap() padding=SpacingSize::Size160.inset()>
                <Flex align=FlexAlign::Center gap=SpacingSize::Size80.flex_gap()>
                    <Icon icon=icondata::AiHistoryOutlined />
                    <InfoLabel>
                        <Caption1 class=class_names.label>"Runs (24h)"</Caption1>
                        <InfoLabelInfo slot>
                            {runs_24h_help()}
                        </InfoLabelInfo>
                    </InfoLabel>
                </Flex>
                <Skeleton>
                    <SkeletonItem class=class_names.value_skeleton size=value_skeleton />
                </Skeleton>
            </Flex>
        </Card>
    }
}

/// Skeleton row matching the four dashboard KPI cards.
#[component]
fn DashboardStatsSkeleton() -> impl IntoView {
    view! {
        <div id="boson-dashboard-stats">
            <Flex gap=SpacingSize::Size160.flex_gap() wrap=FlexWrap::Wrap>
                <div data-testid="dashboard-stat-tasks">
                    <DashboardStatCardSkeleton label="Tasks" icon=icondata::AiAppstoreOutlined />
                </div>
                <div data-testid="dashboard-stat-queued">
                    <DashboardStatCardSkeleton label="Jobs Queued" icon=icondata::AiUnorderedListOutlined />
                </div>
                <div data-testid="dashboard-stat-running">
                    <DashboardStatCardSkeleton label="Jobs Running" icon=icondata::AiThunderboltOutlined />
                </div>
                <div data-testid="dashboard-stat-runs-today">
                    <DashboardHelpStatCardSkeleton />
                </div>
            </Flex>
        </div>
    }
}

/// Loaded KPI cards with staggered enter motion.
#[component]
fn DashboardStatsCards(
    /// Whether enter motion should show.
    kpi_enter: ReadSignal<bool>,
    /// Tasks KPI text.
    task_count: Memo<String>,
    /// Queued jobs KPI text.
    jobs_queued: Memo<String>,
    /// Running jobs KPI text.
    jobs_running: Memo<String>,
    /// Runs-today KPI text.
    runs_today: Memo<String>,
) -> impl IntoView {
    view! {
        <OrbitalPresenceGroup
            motion=boson_kpi_enter_motion()
            stagger=Signal::from(MotionDuration::Normal)
        >
            <div id="boson-dashboard-stats">
                <Flex gap=SpacingSize::Size160.flex_gap() wrap=FlexWrap::Wrap>
                    <OrbitalPresenceGroupItem
                        show=kpi_enter
                        index=Signal::from(0usize)
                    >
                        <div data-testid="dashboard-stat-tasks">
                            <StatCard
                                label="Tasks"
                                value=Signal::derive(move || task_count.get())
                                icon=icondata::AiAppstoreOutlined
                            />
                        </div>
                    </OrbitalPresenceGroupItem>
                    <OrbitalPresenceGroupItem
                        show=kpi_enter
                        index=Signal::from(1usize)
                    >
                        <div data-testid="dashboard-stat-queued">
                            <StatCard
                                label="Jobs Queued"
                                value=Signal::derive(move || jobs_queued.get())
                                icon=icondata::AiUnorderedListOutlined
                            />
                        </div>
                    </OrbitalPresenceGroupItem>
                    <OrbitalPresenceGroupItem
                        show=kpi_enter
                        index=Signal::from(2usize)
                    >
                        <div data-testid="dashboard-stat-running">
                            <StatCard
                                label="Jobs Running"
                                value=Signal::derive(move || jobs_running.get())
                                icon=icondata::AiThunderboltOutlined
                            />
                        </div>
                    </OrbitalPresenceGroupItem>
                    <OrbitalPresenceGroupItem
                        show=kpi_enter
                        index=Signal::from(3usize)
                    >
                        <div data-testid="dashboard-stat-runs-today">
                            <BosonHelpStatCard
                                label="Runs (24h)"
                                value=Signal::derive(move || runs_today.get())
                                icon=icondata::AiHistoryOutlined
                                label_info=runs_24h_help()
                            />
                        </div>
                    </OrbitalPresenceGroupItem>
                </Flex>
            </div>
        </OrbitalPresenceGroup>
    }
}

/// KPI stat cards with staggered enter on first load; poll refetches keep cards mounted.
#[component]
pub fn DashboardStatsGrid(
    /// Resource that loads the stats data.
    stats_res: Resource<Result<DashboardStats, ServerFnError>>,
) -> impl IntoView {
    let kpi_enter = RwSignal::new(false);

    Effect::new(move |_| {
        if stats_res.get().and_then(Result::ok).is_some() {
            kpi_enter.set(true);
        }
    });

    let task_count = Memo::new(move |_| {
        stats_res
            .get()
            .and_then(Result::ok)
            .map(|s| s.task_count.to_string())
            .unwrap_or_default()
    });
    let jobs_queued = Memo::new(move |_| {
        stats_res
            .get()
            .and_then(Result::ok)
            .map(|s| s.jobs_queued.to_string())
            .unwrap_or_default()
    });
    let jobs_running = Memo::new(move |_| {
        stats_res
            .get()
            .and_then(Result::ok)
            .map(|s| s.jobs_running.to_string())
            .unwrap_or_default()
    });
    let runs_today = Memo::new(move |_| {
        stats_res
            .get()
            .and_then(Result::ok)
            .map(|s| s.runs_today.to_string())
            .unwrap_or_default()
    });

    view! {
        <Transition fallback=move || view! { <DashboardStatsSkeleton /> }>
            {move || stats_res.get().map(|r| match r {
                Ok(_) => view! {
                    <DashboardStatsCards
                        kpi_enter=kpi_enter.read_only()
                        task_count=task_count
                        jobs_queued=jobs_queued
                        jobs_running=jobs_running
                        runs_today=runs_today
                    />
                }.into_any(),
                Err(e) => view! {
                    <MessageBar intent=MessageBarIntent::Error>
                        "Failed to load stats: " {e.to_string()}
                    </MessageBar>
                }.into_any(),
            })}
        </Transition>
    }
}
