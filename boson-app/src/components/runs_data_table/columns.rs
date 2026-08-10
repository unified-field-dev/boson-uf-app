use std::sync::Arc;

use leptos::prelude::*;
use orbital::primitives::{ColumnType, DataTableColumnDef};
use orbital_data::DataRecord;

use super::mapper::run_status_from_key;
use crate::components::{
    attempt_help, duration_help, BosonHelpColumnHeader, BosonTableLink, RunStatusBadge,
};

pub fn runs_table_columns() -> Vec<DataTableColumnDef> {
    let status_view = Arc::new(|record: DataRecord| {
        let label = record
            .get("status")
            .map(orbital_data::DataValue::display_string)
            .unwrap_or_default();
        view! { <RunStatusBadge status=run_status_from_key(&label) /> }.into_any()
    });

    let run_id_view = Arc::new(|record: DataRecord| {
        let run_id = record
            .get("run_id")
            .map(orbital_data::DataValue::display_string)
            .unwrap_or_default();
        let testid = format!("runs-row-{run_id}");
        let href = boson_backend::boson_run_path(&run_id);
        view! {
            <div data-testid=testid>
                <BosonTableLink href=href>
                    {run_id.clone()}
                </BosonTableLink>
            </div>
        }
        .into_any()
    });

    let attempt_header = Arc::new(|| {
        view! {
            <BosonHelpColumnHeader label="Attempt" info=attempt_help() />
        }
        .into_any()
    });

    let duration_header = Arc::new(|| {
        view! {
            <BosonHelpColumnHeader label="Duration" info=duration_help() />
        }
        .into_any()
    });

    vec![
        DataTableColumnDef::new("run_id", "Run")
            .with_sortable(false)
            .with_cell_view(run_id_view),
        DataTableColumnDef::new("job_id", "Job").with_sortable(false),
        DataTableColumnDef::new("task_name", "Task").with_sortable(false),
        DataTableColumnDef::new("status", "Status")
            .with_col_type(ColumnType::SingleSelect)
            .with_sortable(false)
            .with_cell_view(status_view),
        DataTableColumnDef::new("attempt", "Attempt")
            .with_sortable(false)
            .with_header_view(attempt_header),
        DataTableColumnDef::new("started_at", "Started").with_sortable(false),
        DataTableColumnDef::new("duration_ms", "Duration")
            .with_sortable(false)
            .with_header_view(duration_header),
    ]
}
