use std::collections::HashSet;
use std::sync::Arc;

use leptos::prelude::*;
use orbital::components::Body1Strong;
use orbital::primitives::*;
use orbital_data::DataRecord;

use crate::components::{pool_help, priority_help, BosonHelpColumnHeader, JobStatusBadge};
use crate::server::JobStatusDto;

use super::mapper::job_status_from_key;

pub fn queue_table_columns(
    cancel_pending: ReadSignal<HashSet<String>>,
    on_cancel: Callback<String>,
) -> Vec<DataTableColumnDef> {
    let status_view = Arc::new(move |record: DataRecord| {
        let label = record
            .get("status")
            .map(orbital_data::DataValue::display_string)
            .unwrap_or_default();
        view! { <JobStatusBadge status=job_status_from_key(&label) /> }.into_any()
    });

    let job_id_view = Arc::new(|record: DataRecord| {
        let job_id = record
            .get("job_id")
            .map(orbital_data::DataValue::display_string)
            .unwrap_or_default();
        let testid = format!("job-card-{job_id}");
        view! {
            <div data-testid=testid>
                <Body1Strong>{job_id}</Body1Strong>
            </div>
        }
        .into_any()
    });

    let pool_header = Arc::new(|| {
        view! {
            <BosonHelpColumnHeader label="Pool" info=pool_help() />
        }
        .into_any()
    });

    let priority_header = Arc::new(|| {
        view! {
            <BosonHelpColumnHeader label="Priority" info=priority_help() />
        }
        .into_any()
    });

    let cancel_view = Arc::new(move |record: DataRecord| {
        let job_id = record
            .get("job_id")
            .map(orbital_data::DataValue::display_string)
            .unwrap_or_default();
        let status_label = record
            .get("status")
            .map(orbital_data::DataValue::display_string)
            .unwrap_or_default();
        let status = job_status_from_key(&status_label);
        let can_cancel = status == JobStatusDto::Queued || status == JobStatusDto::Running;

        if !can_cancel {
            return view! {}.into_any();
        }

        let job_id_cancel = job_id.clone();
        let job_id_check = job_id.clone();
        let job_id_btn = job_id.clone();
        let job_id_label = job_id.clone();
        let on_cancel = on_cancel.clone();

        view! {
            <div data-testid=format!("job-cancel-{job_id_btn}") attr:data-skip-row-click="">
                <Button
                    size=ButtonSize::Small
                    appearance=ButtonAppearance::Subtle
                    disabled=Signal::derive(move || cancel_pending.get().contains(&job_id_check))
                    on_click=Callback::new(move |_| on_cancel.run(job_id_cancel.clone()))
                >
                    {move || {
                        if cancel_pending.get().contains(&job_id_label) {
                            "Cancelling…"
                        } else {
                            "Cancel"
                        }
                    }}
                </Button>
            </div>
        }
        .into_any()
    });

    vec![
        DataTableColumnDef::new("job_id", "Job")
            .with_sortable(false)
            .with_cell_view(job_id_view),
        DataTableColumnDef::new("task_name", "Task").with_sortable(false),
        DataTableColumnDef::new("status", "Status")
            .with_col_type(ColumnType::SingleSelect)
            .with_sortable(false)
            .with_cell_view(status_view),
        DataTableColumnDef::new("pool", "Pool")
            .with_sortable(false)
            .with_header_view(pool_header),
        DataTableColumnDef::new("priority", "Priority")
            .with_sortable(false)
            .with_header_view(priority_header),
        DataTableColumnDef::new("created_at", "Enqueued").with_sortable(false),
        DataTableColumnDef::new("cancel", "")
            .with_sortable(false)
            .with_filterable(false)
            .with_cell_view(cancel_view),
    ]
}
