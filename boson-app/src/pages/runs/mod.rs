use crate::components::{
    boson_table_page_layout, BosonCardContent, RunsDataTable, RunsTableScope,
};
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_query_map};
use leptos_router::NavigateOptions;
use orbital::components::{Card, Caption1, ContentContainer, SpacingSize, Tag, Title3};
use orbital::primitives::{Button, ButtonAppearance, ButtonSize, Flex, FlexAlign};

/// Run history index: paginated list of past runs across all tasks.
#[component]
pub fn BosonRunsIndexPage() -> impl IntoView {
    let query = use_query_map();
    let navigate = use_navigate();
    let navigate_store = StoredValue::new(navigate);

    let job_filter = Memo::new(move |_| {
        query.with(|q| q.get("job").filter(|s| !s.is_empty()))
    });

    let scope = Memo::new(move |_| {
        job_filter
            .get()
            .map_or(RunsTableScope::All, RunsTableScope::ForJob)
    });

    let empty_description = Memo::new(move |_| {
        if job_filter.get().is_some() {
            "No runs found for the selected job."
        } else {
            "Run history will appear here when jobs execute."
        }
    });

    let (page_style, page_classes) = boson_table_page_layout();
    let fill_card_content_store = StoredValue::new(page_classes.card_content.clone());
    let fill_card_store = StoredValue::new(page_classes.card.clone());

    view! {
        <style>{page_style}</style>
        <ContentContainer class=page_classes.page data_testid="boson-runs">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap() class=page_classes.body>
                <Title3>"Runs"</Title3>

                {move || job_filter.get().map(|job_id| {
                    view! {
                        <div data-testid="runs-job-filter-chip">
                            <Flex align=FlexAlign::Center gap=SpacingSize::Size80.flex_gap()>
                                <Caption1>"Filtered by job:"</Caption1>
                                <Tag>{job_id}</Tag>
                                <Button
                                    size=ButtonSize::Small
                                    appearance=ButtonAppearance::Subtle
                                    on_click=Callback::new({
                                        let nav = navigate_store.with_value(Clone::clone);
                                        move |_| {
                                            nav(crate::paths::RUNS, NavigateOptions::default());
                                        }
                                    })
                                >
                                    "Clear"
                                </Button>
                            </Flex>
                        </div>
                    }
                })}

                {move || {
                    let current_scope = scope.get();
                    let desc = empty_description.get();
                    view! {
                        <Card class=fill_card_store.with_value(Clone::clone)>
                            <BosonCardContent class=fill_card_content_store.with_value(Clone::clone)>
                                <RunsDataTable
                                    scope=current_scope
                                    fill_height=true
                                    empty_description=desc
                                />
                            </BosonCardContent>
                        </Card>
                    }
                }}
            </Flex>
        </ContentContainer>
    }
}
