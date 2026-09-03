mod columns;
mod fetcher;
mod mapper;

use std::collections::HashSet;

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use orbital::components::{EmptyState, MessageBar, MessageBarIntent};
use orbital::primitives::{
    DataTable, DataTableEmptyView, DataTableEvents, DataTableFeatures, DataTableHeaderChromeConfig,
    DataTableNoResultsView, DataTableSource, DataTableToolbarConfig, PagingMode,
};

use crate::components::BosonDataTableShell;

use columns::queue_table_columns;
use fetcher::{build_queue_fetcher, QUEUE_TABLE_PAGE_SIZE};

/// Queue jobs DataTable with status filter and cancel actions.
#[component]
pub fn QueueDataTable(
    /// Reactive signal for the cancel pending.
    cancel_pending: ReadSignal<HashSet<String>>,
    /// Callback invoked when the action is cancelled.
    on_cancel: Callback<String>,
    /// Optional refresh signal.
    #[prop(optional)]
    refresh_signal: Option<Signal<u32>>,
) -> impl IntoView {
    let navigate = use_navigate();

    let on_row_click = Callback::new(move |(id,): (String,)| {
        navigate(
            &boson_backend::boson_runs_job_filter_path(&id),
            NavigateOptions::default(),
        );
    });

    let data_source = DataTableSource::Server {
        fetcher: build_queue_fetcher(),
        page_size: QUEUE_TABLE_PAGE_SIZE,
    };

    let table_refresh = refresh_signal.unwrap_or_else(|| Signal::derive(|| 0u32));

    view! {
        <BosonDataTableShell data_testid="boson-queue-data-table">
            <div data-testid="queue-status-filter">
                {move || {
                    let _refresh = table_refresh.get();
                    view! {
                        <DataTable
                            data_source=data_source.clone()
                            paging=PagingMode::Paged
                            flex=true
                            features=DataTableFeatures::MULTI_FILTER
                            columns=queue_table_columns(cancel_pending, on_cancel)
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
                            <DataTableEmptyView slot>
                                <EmptyState
                                    message="No jobs"
                                    description="Jobs will appear here when tasks are enqueued."
                                />
                            </DataTableEmptyView>
                            <DataTableNoResultsView slot>
                                <MessageBar intent=MessageBarIntent::Info>
                                    "No jobs match your search or filters."
                                </MessageBar>
                            </DataTableNoResultsView>
                        </DataTable>
                    }
                }}
            </div>
        </BosonDataTableShell>
    }
}
