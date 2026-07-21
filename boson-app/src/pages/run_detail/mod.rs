mod run_error_display;
mod run_info_grid;

pub use run_error_display::RunErrorDisplay;
pub use run_info_grid::RunInfoGrid;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use orbital::components::{Card, ContentContainer, Skeleton, SkeletonItem, SpacingSize, Title3};
use orbital::primitives::{Flex, FlexAlign, MessageBar, MessageBarIntent};
use orbital_motion::OrbitalPresence;

use crate::components::{boson_error_reveal_motion, BosonBackLink, BosonCardContent};
use crate::live::{
    boson_job_run_subscription, boson_run_event_is_status, boson_run_event_matches_run,
    BosonJobRunLiveSource,
};
use crate::server::{get_run, RunStatusDto};

/// Skeleton placeholder while run detail loads.
#[component]
fn RunDetailSkeleton() -> impl IntoView {
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Panel { width: 100%; height: 240px; }
    };

    view! {
        <style>{style_sheet}</style>
        <Card>
            <BosonCardContent>
                <Skeleton><SkeletonItem class=class_names.panel /></Skeleton>
            </BosonCardContent>
        </Card>
    }
}

/// Detail view for a single run: status, timing, error output, and live updates.
#[component]
pub fn BosonRunDetailPage() -> impl IntoView {
    let params = use_params_map();
    let run_id = move || params.get().get("id").unwrap_or_default();

    let run_res = Resource::new(run_id, move |id| async move {
        if id.is_empty() {
            return Ok(None);
        }
        get_run(id).await
    });

    let live = boson_job_run_subscription();

    let job_id_for_live = Memo::new(move |_| {
        run_res
            .get()
            .and_then(|r| r.ok().flatten())
            .filter(|r| r.status == RunStatusDto::Running)
            .map(|r| r.job_id)
            .filter(|id| !id.is_empty())
    });

    Effect::new(move |_| {
        let _ = live.trigger.get();
        let current_run = run_id();
        if current_run.is_empty() {
            return;
        }
        if let Some(ev) = live.latest_event.get() {
            if boson_run_event_is_status(&ev) && boson_run_event_matches_run(&ev, &current_run) {
                run_res.refetch();
            }
        }
    });

    view! {
        <BosonJobRunLiveSource
            job_id=Signal::derive(move || job_id_for_live.get())
            trigger=live.trigger
            latest_event=live.latest_event
        />
        <ContentContainer data_testid="boson-run-detail">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Flex align=FlexAlign::Center gap=SpacingSize::Size160.flex_gap()>
                    <BosonBackLink
                        href=crate::paths::RUNS
                        label="Back to Runs"
                        data_testid="run-detail-back"
                    />
                    <Title3>"Run: " {run_id}</Title3>
                </Flex>

                <Suspense fallback=move || view! { <RunDetailSkeleton /> }>
                    {move || match run_res.get() {
                        Some(Ok(Some(r))) => {
                            let error_msg = r.error_message.clone();
                            let error_show = Signal::from(error_msg.is_some());
                            view! {
                                <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
                                    <RunInfoGrid run=r />
                                    <OrbitalPresence
                                        show=error_show
                                        motion=boson_error_reveal_motion()
                                        appear=true
                                    >
                                        {move || {
                                            error_msg.clone().map(|msg| {
                                                view! { <RunErrorDisplay message=msg /> }
                                            })
                                        }}
                                    </OrbitalPresence>
                                </Flex>
                            }.into_any()
                        }
                        Some(Ok(None)) => view! {
                            <MessageBar intent=MessageBarIntent::Warning>"Run not found"</MessageBar>
                        }.into_any(),
                        Some(Err(e)) => view! {
                            <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar>
                        }.into_any(),
                        None => view! { <RunDetailSkeleton /> }.into_any(),
                    }}
                </Suspense>
            </Flex>
        </ContentContainer>
    }
}
