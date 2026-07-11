use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use orbital::components::{Body1Strong, Text, TextTag};
use orbital::primitives::*;

/// Action buttons shared by TaskCard and the tasks DataTable actions column.
#[component]
pub fn TaskCardActions(task_name: String) -> impl IntoView {
    let navigate = use_navigate();
    let nav_store = StoredValue::new(navigate);

    let name_view_btn = task_name.clone();
    let name_config = task_name.clone();
    let name_config_btn = task_name.clone();
    let name_card_testid = task_name.clone();

    view! {
        <>
            <div data-testid=format!("task-card-view-{name_card_testid}") attr:data-skip-row-click="">
                <Button
                    size=ButtonSize::Small
                    appearance=ButtonAppearance::Subtle
                    on_click=Callback::new(move |_| {
                        nav_store.with_value(|n| n(&crate::paths::task(&name_view_btn), Default::default()))
                    })
                >
                    "View"
                </Button>
            </div>
            <div data-testid=format!("task-card-config-{name_config}") attr:data-skip-row-click="">
                <Button
                    size=ButtonSize::Small
                    appearance=ButtonAppearance::Subtle
                    on_click=Callback::new(move |_| {
                        nav_store.with_value(|n| {
                            n(&crate::paths::tasks_config(&name_config_btn), Default::default())
                        })
                    })
                >
                    "Configure"
                </Button>
            </div>
            <div attr:data-skip-row-click="">
                <Button
                    size=ButtonSize::Small
                    appearance=ButtonAppearance::Subtle
                    on_click=Callback::new(move |_| {
                        nav_store.with_value(|n| n(crate::paths::QUEUE, Default::default()))
                    })
                >
                    "View Queue"
                </Button>
            </div>
            <div attr:data-skip-row-click="">
                <Button
                    size=ButtonSize::Small
                    appearance=ButtonAppearance::Subtle
                    on_click=Callback::new(move |_| {
                        nav_store.with_value(|n| n(crate::paths::RUNS, Default::default()))
                    })
                >
                    "View Runs"
                </Button>
            </div>
        </>
    }
}
