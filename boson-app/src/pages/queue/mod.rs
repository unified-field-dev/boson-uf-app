mod job_card;

use std::collections::HashSet;

use crate::components::{boson_table_page_layout, BosonCardContent, QueueDataTable};
use crate::live::{boson_job_event_is_status, boson_jobs_subscription, BosonJobsLiveSource};
use crate::server::cancel_job;
use leptos::prelude::*;
use leptos::task::spawn_local;
use orbital::components::{Card, ContentContainer, SpacingSize, Title3};
use orbital::primitives::*;

#[component]
pub fn BosonQueuePage() -> impl IntoView {
    let cancel_pending = RwSignal::new(HashSet::<String>::new());
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
        cancel_pending.update(|s| {
            s.insert(job_id.clone());
        });
        spawn_local(async move {
            let _ = cancel_job(job_id.clone()).await;
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
        <ContentContainer class=page_classes.fill_page data_testid="boson-queue">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap() class=page_classes.fill_body>
                <Title3>"Queue"</Title3>

                <Card class=page_classes.fill_card>
                    <BosonCardContent class=page_classes.fill_card_content>
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
