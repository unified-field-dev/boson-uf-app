use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use orbital::primitives::{Button, ButtonAppearance, ButtonSize};

/// Action buttons shared by TaskCard and the tasks DataTable actions column.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn TaskCardActions(
    /// Task name.
    task_name: String,
    /// When true, attach spotlight tour ids (first table row only).
    #[prop(default = false)]
    spotlight_ids: bool,
) -> impl IntoView {
    let navigate = use_navigate();
    let nav_store = StoredValue::new(navigate);

    let name_view_btn = task_name.clone();
    let name_config = task_name.clone();
    let name_config_btn = task_name.clone();
    let name_card_testid = task_name;

    let view_id = spotlight_ids.then_some("boson-tasks-action-view");
    let configure_id = spotlight_ids.then_some("boson-tasks-action-configure");
    let queue_id = spotlight_ids.then_some("boson-tasks-action-queue");
    let runs_id = spotlight_ids.then_some("boson-tasks-action-runs");

    view! {
        <>
            <div id=view_id data-testid=format!("task-card-view-{name_card_testid}") attr:data-skip-row-click="">
                <Button
                    size=ButtonSize::Small
                    appearance=ButtonAppearance::Subtle
                    on_click=Callback::new(move |_| {
                        nav_store.with_value(|n| n(&boson_backend::boson_task_path(&name_view_btn), NavigateOptions::default()));
                    })
                >
                    "View"
                </Button>
            </div>
            <div id=configure_id data-testid=format!("task-card-config-{name_config}") attr:data-skip-row-click="">
                <Button
                    size=ButtonSize::Small
                    appearance=ButtonAppearance::Subtle
                    on_click=Callback::new(move |_| {
                        nav_store.with_value(|n| {
                            n(&boson_backend::boson_task_config_path(&name_config_btn), NavigateOptions::default());
                        });
                    })
                >
                    "Configure"
                </Button>
            </div>
            <div id=queue_id attr:data-skip-row-click="">
                <Button
                    size=ButtonSize::Small
                    appearance=ButtonAppearance::Subtle
                    on_click=Callback::new(move |_| {
                        nav_store.with_value(|n| n(crate::paths::QUEUE, NavigateOptions::default()));
                    })
                >
                    "View Queue"
                </Button>
            </div>
            <div id=runs_id attr:data-skip-row-click="">
                <Button
                    size=ButtonSize::Small
                    appearance=ButtonAppearance::Subtle
                    on_click=Callback::new(move |_| {
                        nav_store.with_value(|n| n(crate::paths::RUNS, NavigateOptions::default()));
                    })
                >
                    "View Runs"
                </Button>
            </div>
        </>
    }
}
