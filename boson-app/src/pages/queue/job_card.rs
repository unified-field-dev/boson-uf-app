use std::collections::HashSet;

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use orbital::components::{Card, Body1Strong, Caption2, SpacingSize};
use orbital::primitives::*;

use crate::components::BosonCardContent;
use crate::components::JobStatusBadge;
use crate::server::{JobStatusDto, JobSummary};

/// Individual job card with metadata and action buttons.
#[component]
pub fn JobCard(
    job: JobSummary,
    on_cancel: Callback<String>,
    cancel_pending: ReadSignal<HashSet<String>>,
) -> impl IntoView {
    let navigate = use_navigate();
    let nav_store = StoredValue::new(navigate);

    let job_id = job.job_id.clone();
    let job_id_for_testid = job.job_id.clone();
    let job_id_cancel = job.job_id.clone();
    let job_id_cancel_check = job_id_cancel.clone();
    let status = job.status;
    let can_cancel = status == JobStatusDto::Queued || status == JobStatusDto::Running;

    let is_cancelling = Signal::derive(move || cancel_pending.get().contains(&job_id_cancel_check));

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .JobMeta { color: var(--colorNeutralForeground3); }
    };

    view! {
        <style>{style_sheet}</style>
        <div data-testid=format!("job-card-{}", job_id_for_testid)>
            <Card>
                <BosonCardContent>
                    <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                    <Flex justify=FlexJustify::SpaceBetween align=FlexAlign::Center>
                        <Body1Strong>{job.job_id}</Body1Strong>
                        <JobStatusBadge status=status />
                    </Flex>
                    <Caption2 block=true class=class_names.job_meta>
                        "Task: " {job.task_name}
                        "  |  Pool: " {job.pool}
                        "  |  Priority: " {job.priority}
                        "  |  Enqueued: " {job.created_at}
                    </Caption2>
                    <Flex gap=SpacingSize::Size80.flex_gap()>
                        <Button
                            size=ButtonSize::Small
                            appearance=ButtonAppearance::Subtle
                            on_click=Callback::new(move |_| {
                                nav_store.with_value(|n| {
                                    n(
                                        &format!("{}?job={}", crate::paths::RUNS, job_id),
                                        Default::default(),
                                    )
                                })
                            })
                        >
                            "View Runs"
                        </Button>
                        {if can_cancel {
                            let job_id_cancel_btn = job_id_cancel.clone();
                            view! {
                                <div data-testid=format!("job-cancel-{}", job_id_cancel_btn)>
                                    <Button
                                        size=ButtonSize::Small
                                        appearance=ButtonAppearance::Subtle
                                        disabled=is_cancelling
                                        on_click=Callback::new(move |_| on_cancel.run(job_id_cancel.clone()))
                                    >
                                        {move || if is_cancelling.get() { "Cancelling…" } else { "Cancel" }}
                                    </Button>
                                </div>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }}
                    </Flex>
                    </Flex>
                </BosonCardContent>
            </Card>
        </div>
    }
}
