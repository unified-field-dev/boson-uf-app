mod task_card;

pub use task_card::TaskCard;

use crate::components::{boson_table_page_layout, BosonCardContent, TasksDataTable};
use leptos::prelude::*;
use orbital::components::{Card, ContentContainer, SpacingSize, Title3};
use orbital::primitives::*;

#[component]
pub fn BosonTasksIndexPage() -> impl IntoView {
    let (page_style, page_classes) = boson_table_page_layout();

    view! {
        <style>{page_style}</style>
        <ContentContainer class=page_classes.fill_page data_testid="boson-tasks">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap() class=page_classes.fill_body>
                <Title3>"Tasks"</Title3>

                <Card class=page_classes.fill_card>
                    <BosonCardContent class=page_classes.fill_card_content>
                        <div data-testid="tasks-search">
                            <TasksDataTable />
                        </div>
                    </BosonCardContent>
                </Card>
            </Flex>
        </ContentContainer>
    }
}
