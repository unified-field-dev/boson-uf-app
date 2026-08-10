mod basic_config_form;
mod form_state;
mod retry_policy_form;

pub use basic_config_form::TaskConfigForm;
pub use retry_policy_form::RetryPolicyForm;

use form_state::use_task_config_form;

use leptos::prelude::*;
use leptos::server_fn::ServerFnError;
use leptos_router::hooks::use_navigate;
use leptos_router::hooks::use_params_map;
use leptos_router::NavigateOptions;
use orbital::components::{ContentContainer, Skeleton, SkeletonItem, SpacingSize, Title3};
use orbital::primitives::{Button, ButtonAppearance, Flex, MessageBar, MessageBarIntent};

use crate::components::BosonBackLink;
use crate::server::{get_task_config, list_gluon_pools_for_boson_task_config};

/// Skeleton placeholder while task config form loads.
#[component]
fn TaskConfigSkeleton() -> impl IntoView {
    view! {
        <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
            <Skeleton>
                <SkeletonItem width="100%".to_string() height="160px".to_string() />
            </Skeleton>
            <Skeleton>
                <SkeletonItem width="100%".to_string() height="160px".to_string() />
            </Skeleton>
            <Skeleton>
                <SkeletonItem width="30%".to_string() height="32px".to_string() />
            </Skeleton>
        </Flex>
    }
}

/// Task configuration form: priority, pools, and retry policy. Requires verified email
/// (see the `BosonVerifiedTaskConfigPage` route guard in the crate root).
#[component]
pub fn BosonTaskConfigPage() -> impl IntoView {
    let params = use_params_map();
    let task_name = move || params.get().get("task_name").unwrap_or_default();
    let navigate = use_navigate();
    let nav_store = StoredValue::new(navigate);

    let config_res = Resource::new(task_name, move |name| async move {
        if name.is_empty() {
            return Err(ServerFnError::new("Missing task name"));
        }
        get_task_config(name).await
    });

    let pools_res = Resource::new(|| (), |()| list_gluon_pools_for_boson_task_config());

    let task_name_memo = Memo::new(move |_| task_name());
    let form = use_task_config_form(config_res);
    let on_save = form.save_callback(task_name_memo, config_res);

    view! {
        <ContentContainer data_testid="boson-task-config">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <BosonBackLink href=crate::paths::TASKS label="Back to Tasks" />
                <Title3>"Configure Task: " {task_name}</Title3>

                <Suspense fallback=move || view! { <TaskConfigSkeleton /> }>
                    {move || match (config_res.get(), pools_res.get()) {
                        (Some(Ok(_)), pools) => {
                            let pool_options = pools.and_then(Result::ok).unwrap_or_default();
                            view! {
                            <TaskConfigForm
                                pool=form.pool
                                priority_str=form.priority_str
                                pool_options=pool_options
                            />
                            <RetryPolicyForm
                                max_attempts_str=form.max_attempts_str
                                base_delay_ms_str=form.base_delay_ms_str
                                max_delay_ms_str=form.max_delay_ms_str
                                backoff_multiplier_str=form.backoff_multiplier_str
                            />
                            {move || form.save_error.get().map_or_else(
                                || {
                                    let _: () = view! {};
                                    ().into_any()
                                },
                                |e| view! {
                                    <MessageBar intent=MessageBarIntent::Error>{e}</MessageBar>
                                }.into_any(),
                            )}
                            <Flex gap=SpacingSize::Size120.flex_gap()>
                                <Button
                                    appearance=ButtonAppearance::Subtle
                                    on_click=Callback::new(move |_| { nav_store.with_value(|n| n(crate::paths::TASKS, NavigateOptions::default())); })
                                >
                                    "Cancel"
                                </Button>
                                <div data-testid="task-config-save">
                                    <Button
                                        appearance=ButtonAppearance::Primary
                                        disabled=form.save_pending
                                        on_click=on_save
                                    >
                                        {move || if form.save_pending.get() { "Saving..." } else { "Save Changes" }}
                                    </Button>
                                </div>
                            </Flex>
                        }
                            .into_any()
                        }
                        (Some(Err(e)), _) => view! {
                            <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar>
                        }.into_any(),
                        (None, _) => view! { <TaskConfigSkeleton /> }.into_any(),
                    }}
                </Suspense>
            </Flex>
        </ContentContainer>
    }
}
