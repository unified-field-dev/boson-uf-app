//! Input validation gates for Boson job/run/task server lookups.

/// Rejects blank task names before registry / config lookups.
///
/// # Errors
///
/// Returns an error message when `task_name` is empty or whitespace-only.
pub fn validate_task_name(task_name: &str) -> Result<(), String> {
    if task_name.trim().is_empty() {
        Err("Boson task name is required".to_string())
    } else {
        Ok(())
    }
}

/// Rejects blank job ids before cancel / queue lookups.
///
/// # Errors
///
/// Returns an error message when `job_id` is empty or whitespace-only.
pub fn validate_job_id(job_id: &str) -> Result<(), String> {
    if job_id.trim().is_empty() {
        Err("Boson job id is required".to_string())
    } else {
        Ok(())
    }
}

/// Rejects blank run ids before run detail lookups.
///
/// # Errors
///
/// Returns an error message when `run_id` is empty or whitespace-only.
pub fn validate_run_id(run_id: &str) -> Result<(), String> {
    if run_id.trim().is_empty() {
        Err("Boson run id is required".to_string())
    } else {
        Ok(())
    }
}
