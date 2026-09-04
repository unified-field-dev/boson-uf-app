mod skeleton;

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::hooks::use_params_map;
use leptos_router::NavigateOptions;
use orbital::components::{Card, ContentContainer, SpacingSize, Title3};
use orbital::primitives::{
    Button, ButtonAppearance, Flex, FlexAlign, MessageBar, MessageBarIntent,
};

use crate::components::{BosonBackLink, BosonCardContent, TaskSummaryPanel};
use crate::server::get_task;

use skeleton::TaskDetailSkeleton;

/// Detail view for a single task: configuration summary and recent runs.
#[component]
pub fn BosonTaskDetailPage() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();
    let navigate_store = StoredValue::new(navigate);
    let task_name = move || params.get().get("task_name").unwrap_or_default();
    let task_res = Resource::new(task_name, move |name| async move {
        if name.is_empty() {
            return Ok(None);
        }
        get_task(name).await
    });

    view! {
        <ContentContainer data_testid="boson-task-detail">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Flex align=FlexAlign::Center gap=SpacingSize::Size160.flex_gap()>
                    <BosonBackLink href=crate::paths::TASKS label="Back to Tasks" />
                    <Title3>"Task: " {task_name}</Title3>
                </Flex>

                <Suspense fallback=move || view! { <TaskDetailSkeleton /> }>
                    {move || match task_res.get() {
                        Some(Ok(Some(t))) => {
                            let config_path = boson_backend::boson_task_config_path(&t.name);
                            let nav = navigate_store.with_value(Clone::clone);
                            let nav2 = navigate_store.with_value(Clone::clone);
                            let nav3 = navigate_store.with_value(Clone::clone);
                            view! {
                                <Card>
                                    <BosonCardContent>
                                        <TaskSummaryPanel task=t show_title=false>
                                            <div data-testid="task-detail-configure">
                                                <Button appearance=ButtonAppearance::Primary on_click=Callback::new(move |_| nav(&config_path, NavigateOptions::default()))>"Configure"</Button>
                                            </div>
                                            <Button appearance=ButtonAppearance::Subtle on_click=Callback::new(move |_| nav2(crate::paths::QUEUE, NavigateOptions::default()))>"View Queue"</Button>
                                            <Button appearance=ButtonAppearance::Subtle on_click=Callback::new(move |_| nav3(crate::paths::RUNS, NavigateOptions::default()))>"View Runs"</Button>
                                        </TaskSummaryPanel>
                                    </BosonCardContent>
                                </Card>
                            }.into_any()
                        }
                        Some(Ok(None)) => view! {
                            <MessageBar intent=MessageBarIntent::Warning>"Task not found"</MessageBar>
                        }.into_any(),
                        Some(Err(e)) => view! {
                            <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar>
                        }.into_any(),
                        None => view! { <TaskDetailSkeleton /> }.into_any(),
                    }}
                </Suspense>
            </Flex>
        </ContentContainer>
    }
}
