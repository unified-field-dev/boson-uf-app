mod actions;
mod columns;
mod fetcher;
mod mapper;

pub use actions::TaskCardActions;

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use orbital::components::{EmptyState, MessageBar, MessageBarIntent};
use orbital::primitives::{
    DataTable, DataTableEmptyView, DataTableEvents, DataTableFeatures, DataTableHeaderChromeConfig,
    DataTableLoadingView, DataTableNoResultsView, DataTableSource, DataTableToolbarConfig,
    ListViewConfig, PagingMode,
};

use crate::components::{BosonDataTableRefetchSkeleton, BosonDataTableShell};

use columns::tasks_table_columns;
use fetcher::{build_tasks_fetcher, TASKS_TABLE_PAGE_SIZE};

/// Tasks list DataTable with LIST_VIEW, search, and filters.
#[component]
pub fn TasksDataTable() -> impl IntoView {
    let navigate = use_navigate();

    let on_row_click = Callback::new(move |(id,): (String,)| {
        navigate(
            &boson_backend::boson_task_path(&id),
            NavigateOptions::default(),
        );
    });

    let data_source = DataTableSource::Server {
        fetcher: build_tasks_fetcher(),
        page_size: TASKS_TABLE_PAGE_SIZE,
    };

    view! {
        <BosonDataTableShell id="boson-tasks-data-table" data_testid="boson-tasks-data-table">
            <DataTable
                data_source=data_source
                paging=PagingMode::Paged
                flex=true
                features=DataTableFeatures::LIST_VIEW | DataTableFeatures::MULTI_FILTER
                list_view=ListViewConfig::new("name")
                    .with_secondary_fields(vec![
                        "effective_pool".into(),
                        "jobs_queued".into(),
                        "success_rate".into(),
                    ])
                columns=tasks_table_columns()
                sortable=false
                toolbar_config=DataTableToolbarConfig {
                    quick_search: true,
                    // P7: filter panel motion deferred — DataTableFilterPanel owns Popover
                    // internally; track upstream orbital-datagrid filter/Popover motion hook.
                    filter_panel: true,
                    column_picker: false,
                    pivot: false,
                    export_menu: false,
                }
                header_chrome=DataTableHeaderChromeConfig {
                    column_menu: false,
                    column_filter_button: false,
                    column_hide: false,
                }
                events=DataTableEvents {
                    on_row_click: Some(on_row_click),
                    ..Default::default()
                }
            >
                <DataTableLoadingView slot>
                    <BosonDataTableRefetchSkeleton />
                </DataTableLoadingView>
                <DataTableEmptyView slot>
                    <EmptyState
                        message="No tasks"
                        description="Register tasks with #[boson::task] macro to see them here."
                    />
                </DataTableEmptyView>
                <DataTableNoResultsView slot>
                    <MessageBar intent=MessageBarIntent::Info>
                        "No tasks match your search or filters."
                    </MessageBar>
                </DataTableNoResultsView>
            </DataTable>
        </BosonDataTableShell>
    }
}
