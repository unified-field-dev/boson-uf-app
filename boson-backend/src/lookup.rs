//! In-memory list/detail contracts for task/job/run collections.

use crate::types::{JobSummary, RunSummary, TaskSummary};

/// Locates a task summary by exact name (used by `get_task` detail lookups).
#[must_use]
pub fn find_task_by_name<'a>(tasks: &'a [TaskSummary], task_name: &str) -> Option<&'a TaskSummary> {
    tasks.iter().find(|t| t.name == task_name)
}

/// Locates a job summary by exact id (used by cancel / queue detail contracts).
#[must_use]
pub fn find_job_by_id<'a>(jobs: &'a [JobSummary], job_id: &str) -> Option<&'a JobSummary> {
    jobs.iter().find(|j| j.job_id == job_id)
}

/// Locates a run summary by exact id (used by `get_run` detail lookups).
#[must_use]
pub fn find_run_by_id<'a>(runs: &'a [RunSummary], run_id: &str) -> Option<&'a RunSummary> {
    runs.iter().find(|r| r.run_id == run_id)
}

/// Sorts task summaries by name (stable list contract for `get_tasks`).
pub fn sort_tasks_by_name(tasks: &mut [TaskSummary]) {
    tasks.sort_by(|a, b| a.name.cmp(&b.name));
}

/// Filters tasks by case-insensitive substring match on name, signature, or pool.
///
/// Blank / whitespace-only queries leave the list unchanged.
pub fn filter_tasks_by_query(tasks: &mut Vec<TaskSummary>, query: Option<&str>) {
    let Some(q) = query else {
        return;
    };
    let q_lower = q.trim().to_lowercase();
    if q_lower.is_empty() {
        return;
    }
    tasks.retain(|t| {
        t.name.to_lowercase().contains(&q_lower)
            || t.signature_json.to_lowercase().contains(&q_lower)
            || t.effective_pool.to_lowercase().contains(&q_lower)
    });
}
