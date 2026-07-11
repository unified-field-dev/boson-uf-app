mod columns;
mod fetcher;
mod mapper;

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use orbital::components::{EmptyState, MessageBar, MessageBarIntent};
use orbital::primitives::*;

use crate::components::BosonDataTableShell;

use columns::runs_table_columns;
use fetcher::{build_runs_fetcher, RUNS_TABLE_PAGE_SIZE};

/// Scope for the Boson runs DataTable.
#[derive(Clone, PartialEq, Eq)]
pub enum RunsTableScope {
    All,
    ForJob(String),
}

/// Runs list DataTable with search, filters, and table layout.
#[component]
pub fn RunsDataTable(
    scope: RunsTableScope,
    #[prop(default = RUNS_TABLE_PAGE_SIZE)] page_size: u32,
    #[prop(default = true)] fill_height: bool,
    #[prop(optional)] empty_description: Option<&'static str>,
) -> impl IntoView {
    let navigate = use_navigate();

    let on_row_click = Callback::new(move |(id,): (String,)| {
        let _ = navigate(&crate::paths::run(&id), Default::default());
    });

    let data_source = DataTableSource::Server {
        fetcher: build_runs_fetcher(scope),
        page_size,
    };

    let empty_desc = empty_description
        .unwrap_or("Run history will appear here when jobs execute.");

    view! {
        <BosonDataTableShell data_testid="boson-runs-data-table">
            <DataTable
                data_source=data_source
                paging=PagingMode::Paged
                flex=fill_height
                features=DataTableFeatures::MULTI_FILTER
                columns=runs_table_columns()
                sortable=false
                toolbar_config=DataTableToolbarConfig {
                    quick_search: true,
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
                <DataTableEmptyView slot>
                    <EmptyState
                        message="No runs"
                        description=empty_desc
                    />
                </DataTableEmptyView>
                <DataTableNoResultsView slot>
                    <MessageBar intent=MessageBarIntent::Info>
                        "No runs match your search or filters."
                    </MessageBar>
                </DataTableNoResultsView>
            </DataTable>
        </BosonDataTableShell>
    }
}
