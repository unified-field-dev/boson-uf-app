mod task_card;

pub use task_card::TaskCard;

use crate::components::{boson_table_page_layout, BosonCardContent, TasksDataTable};
use leptos::prelude::*;
use orbital::components::{Card, ContentContainer, SpacingSize, Title3};
use orbital::primitives::Flex;

/// Task index: searchable list of all configured tasks.
#[component]
pub fn BosonTasksIndexPage() -> impl IntoView {
    let (page_style, page_classes) = boson_table_page_layout();

    view! {
        <style>{page_style}</style>
        <div id="boson-tasks">
            <ContentContainer class=page_classes.page data_testid="boson-tasks">
                <Flex vertical=true gap=SpacingSize::Size240.flex_gap() class=page_classes.body>
                    <Title3>"Tasks"</Title3>

                    <Card class=page_classes.card>
                        <BosonCardContent class=page_classes.card_content>
                            <div id="boson-tasks-search" data-testid="tasks-search">
                                <TasksDataTable />
                            </div>
                        </BosonCardContent>
                    </Card>
                </Flex>
            </ContentContainer>
        </div>
    }
}
