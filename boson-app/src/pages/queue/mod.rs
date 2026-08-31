use std::collections::HashSet;

use crate::components::{boson_table_page_layout, BosonCardContent, QueueDataTable};
use crate::live::{boson_job_event_is_status, boson_jobs_subscription, BosonJobsLiveSource};
use crate::server::cancel_job;
use leptos::prelude::*;
use leptos::task::spawn_local_scoped;
use orbital::components::{Card, ContentContainer, SpacingSize, Title3};
use orbital::primitives::{Flex, MessageBar, MessageBarIntent};

/// Job queue view: pending/active jobs across all tasks.
#[component]
pub fn BosonQueuePage() -> impl IntoView {
    let cancel_pending = RwSignal::new(HashSet::<String>::new());
    let cancel_error = RwSignal::new(None::<String>);
    let trigger_refetch = RwSignal::new(0u32);
    let refresh_signal = Signal::derive(move || trigger_refetch.get());
    let live = boson_jobs_subscription();

    Effect::new(move |_| {
        let _ = live.trigger.get();
        let Some(ev) = live.latest_event.get() else {
            return;
        };
        if boson_job_event_is_status(&ev) {
            trigger_refetch.update(|n| *n += 1);
        }
    });

    let on_cancel = Callback::new(move |job_id: String| {
        if cancel_pending.get().contains(&job_id) {
            return;
        }
        cancel_error.set(None);
        cancel_pending.update(|s| {
            s.insert(job_id.clone());
        });
        spawn_local_scoped(async move {
            match cancel_job(job_id.clone()).await {
                Ok(()) => cancel_error.set(None),
                Err(e) => cancel_error.set(Some(e.to_string())),
            }
            cancel_pending.update(|s| {
                s.remove(&job_id);
            });
            trigger_refetch.update(|n| *n += 1);
        });
    });

    let (page_style, page_classes) = boson_table_page_layout();

    view! {
        <style>{page_style}</style>
        <BosonJobsLiveSource trigger=live.trigger latest_event=live.latest_event />
        <ContentContainer class=page_classes.page data_testid="boson-queue">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap() class=page_classes.body>
                <Title3>"Queue"</Title3>
                {move || cancel_error.get().map(|e| {
                    view! {
                        <MessageBar intent=MessageBarIntent::Error>{e}</MessageBar>
                    }
                })}

                <Card class=page_classes.card>
                    <BosonCardContent class=page_classes.card_content>
                        <QueueDataTable
                            cancel_pending=cancel_pending.read_only()
                            on_cancel=on_cancel
                            refresh_signal=refresh_signal
                        />
                    </BosonCardContent>
                </Card>
            </Flex>
        </ContentContainer>
    }
}
