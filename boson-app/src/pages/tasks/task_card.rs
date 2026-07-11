use leptos::prelude::*;
use orbital::components::Card;

use crate::components::{BosonCardContent, TaskCardActions, TaskSummaryPanel};
use crate::server::TaskSummary;

/// Individual task card with metadata and action buttons.
#[component]
pub fn TaskCard(task: TaskSummary) -> impl IntoView {
    let name = task.name.clone();
    let name_card_testid = name.clone();

    view! {
        <div data-testid=format!("task-{}", name_card_testid)>
            <Card>
                <BosonCardContent>
                    <TaskSummaryPanel task=task show_title=true>
                        <TaskCardActions task_name=name />
                    </TaskSummaryPanel>
                </BosonCardContent>
            </Card>
        </div>
    }
}
