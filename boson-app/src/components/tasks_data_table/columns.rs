use std::sync::Arc;

use leptos::prelude::*;
use orbital::components::{Body1Strong, SpacingSize, Text, TextTag};
use orbital::primitives::{DataTableColumnDef, Flex, FlexWrap};
use orbital_data::DataRecord;

use crate::components::{
    defaults_help, effective_help, signature_help, success_rate_help, BosonHelpColumnHeader,
};

use super::actions::TaskCardActions;
use super::mapper::record_to_task_name;

pub fn tasks_table_columns() -> Vec<DataTableColumnDef> {
    let name_view = Arc::new(|record: DataRecord| {
        let name = record_to_task_name(&record);
        let testid = format!("task-{name}");
        view! {
            <div data-testid=testid>
                <Body1Strong>{name}</Body1Strong>
            </div>
        }
        .into_any()
    });

    let signature_view = Arc::new(|record: DataRecord| {
        let sig = record
            .get("signature")
            .map(orbital_data::DataValue::display_string)
            .unwrap_or_default();
        view! {
            <Text tag=TextTag::Code>{sig}</Text>
        }
        .into_any()
    });

    let signature_header = Arc::new(|| {
        view! {
            <BosonHelpColumnHeader label="Signature" info=signature_help() />
        }
        .into_any()
    });

    let effective_header = Arc::new(|| {
        view! {
            <BosonHelpColumnHeader label="Effective" info=effective_help() />
        }
        .into_any()
    });

    let defaults_header = Arc::new(|| {
        view! {
            <BosonHelpColumnHeader label="Defaults" info=defaults_help() />
        }
        .into_any()
    });

    let success_header = Arc::new(|| {
        view! {
            <BosonHelpColumnHeader label="Success Rate" info=success_rate_help() />
        }
        .into_any()
    });

    let actions_view = Arc::new(|record: DataRecord| {
        let name = record_to_task_name(&record);
        view! {
            <Flex gap=SpacingSize::Size40.flex_gap() wrap=FlexWrap::Wrap attr:data-skip-row-click="">
                <TaskCardActions task_name=name />
            </Flex>
        }
        .into_any()
    });

    vec![
        DataTableColumnDef::new("name", "Task")
            .with_sortable(false)
            .with_cell_view(name_view),
        DataTableColumnDef::new("signature", "Signature")
            .with_sortable(false)
            .with_header_view(signature_header)
            .with_cell_view(signature_view),
        DataTableColumnDef::new("effective_pool", "Effective")
            .with_sortable(false)
            .with_header_view(effective_header),
        DataTableColumnDef::new("default_pool", "Defaults")
            .with_sortable(false)
            .with_header_view(defaults_header),
        DataTableColumnDef::new("jobs_queued", "Queued").with_sortable(false),
        DataTableColumnDef::new("runs_total", "Runs").with_sortable(false),
        DataTableColumnDef::new("success_rate", "Success Rate")
            .with_sortable(false)
            .with_header_view(success_header),
        DataTableColumnDef::new("actions", "Actions")
            .with_sortable(false)
            .with_filterable(false)
            .with_cell_view(actions_view),
    ]
}
