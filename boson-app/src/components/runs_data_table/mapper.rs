use std::collections::HashMap;

use orbital_data::{DataRecord, DataValue};

use crate::server::{RunStatusDto, RunSummary};
use crate::server::page_query::run_status_key;

fn format_started_at(iso: &str) -> String {
    if let Some(dt_part) = iso.split('T').nth(1) {
        if let Some(time_str) = dt_part.split('.').next() {
            let parts: Vec<&str> = time_str.trim_end_matches('Z').split(':').collect();
            if parts.len() >= 2 {
                return format!("{}:{}", parts[0], parts[1]);
            }
        }
    }
    iso.to_string()
}

fn format_duration(ms: Option<i64>) -> String {
    ms.map(|v| format!("{v}ms")).unwrap_or_else(|| "-".to_string())
}

pub fn run_to_record(run: RunSummary) -> DataRecord {
    let id = run.run_id.clone();
    DataRecord::new(
        id,
        HashMap::from([
            ("run_id".into(), DataValue::Text(run.run_id)),
            ("job_id".into(), DataValue::Text(run.job_id)),
            ("task_name".into(), DataValue::Text(run.task_name)),
            (
                "status".into(),
                DataValue::Text(run_status_key(run.status).into()),
            ),
            ("attempt".into(), DataValue::Text(run.attempt.to_string())),
            (
                "started_at".into(),
                DataValue::Text(format_started_at(&run.started_at)),
            ),
            (
                "duration_ms".into(),
                DataValue::Text(format_duration(run.duration_ms)),
            ),
        ]),
    )
}

pub fn run_status_from_key(key: &str) -> RunStatusDto {
    match key {
        "running" => RunStatusDto::Running,
        "success" => RunStatusDto::Success,
        "failed" => RunStatusDto::Failed,
        "canceled" => RunStatusDto::Canceled,
        "timeout" => RunStatusDto::Timeout,
        _ => RunStatusDto::Running,
    }
}
