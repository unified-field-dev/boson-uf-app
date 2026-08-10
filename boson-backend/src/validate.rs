//! Input validation gates for Boson job/run/task server lookups.

/// Blank task name, job id, or run id rejected before Boson lookups.
///
/// Callers map this into Leptos `ServerFnError` (or equivalent) at the `#[server]`
/// boundary; the Display text stays stable for UI and contract tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum BosonIdError {
    /// Task name was empty or whitespace-only.
    EmptyTaskName,
    /// Job id was empty or whitespace-only.
    EmptyJobId,
    /// Run id was empty or whitespace-only.
    EmptyRunId,
}

impl std::fmt::Display for BosonIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyTaskName => write!(f, "Boson task name is required"),
            Self::EmptyJobId => write!(f, "Boson job id is required"),
            Self::EmptyRunId => write!(f, "Boson run id is required"),
        }
    }
}

impl std::error::Error for BosonIdError {}

/// Rejects blank task names before registry / config lookups.
///
/// # Errors
///
/// Returns [`BosonIdError::EmptyTaskName`] when `task_name` is empty or whitespace-only.
pub fn validate_task_name(task_name: &str) -> Result<(), BosonIdError> {
    if task_name.trim().is_empty() {
        Err(BosonIdError::EmptyTaskName)
    } else {
        Ok(())
    }
}

/// Rejects blank job ids before cancel / queue lookups.
///
/// # Errors
///
/// Returns [`BosonIdError::EmptyJobId`] when `job_id` is empty or whitespace-only.
pub fn validate_job_id(job_id: &str) -> Result<(), BosonIdError> {
    if job_id.trim().is_empty() {
        Err(BosonIdError::EmptyJobId)
    } else {
        Ok(())
    }
}

/// Rejects blank run ids before run detail lookups.
///
/// # Errors
///
/// Returns [`BosonIdError::EmptyRunId`] when `run_id` is empty or whitespace-only.
pub fn validate_run_id(run_id: &str) -> Result<(), BosonIdError> {
    if run_id.trim().is_empty() {
        Err(BosonIdError::EmptyRunId)
    } else {
        Ok(())
    }
}
