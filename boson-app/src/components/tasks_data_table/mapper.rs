use std::collections::HashMap;

use orbital_data::{DataRecord, DataValue};

use crate::server::TaskSummary;

pub fn task_to_record(task: TaskSummary) -> DataRecord {
    let id = task.name.clone();
    let success_rate = task
        .success_rate_pct
        .map_or_else(|| "-".to_string(), |r| format!("{r:.0}%"));
    let effective_pool = format!(
        "pool=\"{}\", priority={}",
        task.effective_pool, task.effective_priority
    );
    let default_pool = format!(
        "pool=\"{}\", priority={}",
        task.default_pool, task.default_priority
    );

    DataRecord::new(
        id,
        HashMap::from([
            ("name".into(), DataValue::Text(task.name)),
            ("signature".into(), DataValue::Text(task.signature_json)),
            ("effective_pool".into(), DataValue::Text(effective_pool)),
            ("default_pool".into(), DataValue::Text(default_pool)),
            (
                "jobs_queued".into(),
                DataValue::Text(task.jobs_queued.to_string()),
            ),
            (
                "runs_total".into(),
                DataValue::Text(task.runs_total.to_string()),
            ),
            ("success_rate".into(), DataValue::Text(success_rate)),
        ]),
    )
}

pub fn record_to_task_name(record: &DataRecord) -> String {
    record
        .get("name")
        .map(orbital_data::DataValue::display_string)
        .unwrap_or_default()
}
