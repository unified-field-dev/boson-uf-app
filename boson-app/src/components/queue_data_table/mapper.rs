use std::collections::HashMap;

use orbital_data::{DataRecord, DataValue};

use crate::server::{JobStatusDto, JobSummary};
use crate::server::page_query::job_status_key;

fn format_enqueued(iso: &str) -> String {
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

pub fn job_to_record(job: JobSummary) -> DataRecord {
    let id = job.job_id.clone();
    DataRecord::new(
        id,
        HashMap::from([
            ("job_id".into(), DataValue::Text(job.job_id)),
            ("task_name".into(), DataValue::Text(job.task_name)),
            (
                "status".into(),
                DataValue::Text(job_status_key(job.status).into()),
            ),
            ("pool".into(), DataValue::Text(job.pool)),
            ("priority".into(), DataValue::Text(job.priority.to_string())),
            (
                "created_at".into(),
                DataValue::Text(format_enqueued(&job.created_at)),
            ),
        ]),
    )
}

pub fn job_status_from_key(key: &str) -> JobStatusDto {
    match key {
        "queued" => JobStatusDto::Queued,
        "running" => JobStatusDto::Running,
        "success" => JobStatusDto::Success,
        "failed" => JobStatusDto::Failed,
        "canceled" => JobStatusDto::Canceled,
        _ => JobStatusDto::Queued,
    }
}
