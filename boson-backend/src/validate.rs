//! Blank-id rejection, unsafe-id rejection, and path-segment encoding for ops
//! UI hrefs.

/// Blank, oversized, or path-unsafe task name, job id, or run id rejected before
/// Boson lookups.
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
    /// Task name exceeded [`MAX_BOSON_ID_CHARS`].
    TaskNameTooLong,
    /// Job id exceeded [`MAX_BOSON_ID_CHARS`].
    JobIdTooLong,
    /// Run id exceeded [`MAX_BOSON_ID_CHARS`].
    RunIdTooLong,
    /// Task name contained `/`, `\`, controls, or was `.` / `..`.
    UnsafeTaskName,
    /// Job id contained `/`, `\`, controls, or was `.` / `..`.
    UnsafeJobId,
    /// Run id contained `/`, `\`, controls, or was `.` / `..`.
    UnsafeRunId,
}

impl std::fmt::Display for BosonIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyTaskName => write!(f, "Boson task name is required"),
            Self::EmptyJobId => write!(f, "Boson job id is required"),
            Self::EmptyRunId => write!(f, "Boson run id is required"),
            Self::TaskNameTooLong => write!(f, "Boson task name is too long"),
            Self::JobIdTooLong => write!(f, "Boson job id is too long"),
            Self::RunIdTooLong => write!(f, "Boson run id is too long"),
            Self::UnsafeTaskName => {
                write!(f, "Boson task name contains unsafe path characters")
            }
            Self::UnsafeJobId => {
                write!(f, "Boson job id contains unsafe path characters")
            }
            Self::UnsafeRunId => {
                write!(f, "Boson run id contains unsafe path characters")
            }
        }
    }
}

impl std::error::Error for BosonIdError {}

/// Maximum Unicode scalar count for task names, job ids, and run ids accepted by
/// ops detail lookups.
pub const MAX_BOSON_ID_CHARS: usize = 256;

const fn is_unsafe_ops_id_char(c: char) -> bool {
    c.is_control() || c == '/' || c == '\\'
}

fn check_ops_id(raw: &str) -> Result<&str, BosonIdErrorKind> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(BosonIdErrorKind::Empty);
    }
    if trimmed.chars().count() > MAX_BOSON_ID_CHARS {
        return Err(BosonIdErrorKind::TooLong);
    }
    if trimmed == "." || trimmed == ".." {
        return Err(BosonIdErrorKind::Unsafe);
    }
    if trimmed.chars().any(is_unsafe_ops_id_char) {
        return Err(BosonIdErrorKind::Unsafe);
    }
    Ok(trimmed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BosonIdErrorKind {
    Empty,
    TooLong,
    Unsafe,
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` task names
/// before registry / config lookups.
///
/// # Errors
///
/// Returns a [`BosonIdError`] variant when the name is empty/whitespace-only,
/// longer than [`MAX_BOSON_ID_CHARS`], contains `/` `\` or ASCII controls, or is
/// exactly `.` / `..`.
pub fn validate_task_name(task_name: &str) -> Result<(), BosonIdError> {
    match check_ops_id(task_name) {
        Ok(_) => Ok(()),
        Err(BosonIdErrorKind::Empty) => Err(BosonIdError::EmptyTaskName),
        Err(BosonIdErrorKind::TooLong) => Err(BosonIdError::TaskNameTooLong),
        Err(BosonIdErrorKind::Unsafe) => Err(BosonIdError::UnsafeTaskName),
    }
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` job ids
/// before cancel / queue / filter lookups.
///
/// # Errors
///
/// Returns a [`BosonIdError`] variant when the id fails the same rules as
/// [`validate_task_name`].
pub fn validate_job_id(job_id: &str) -> Result<(), BosonIdError> {
    match check_ops_id(job_id) {
        Ok(_) => Ok(()),
        Err(BosonIdErrorKind::Empty) => Err(BosonIdError::EmptyJobId),
        Err(BosonIdErrorKind::TooLong) => Err(BosonIdError::JobIdTooLong),
        Err(BosonIdErrorKind::Unsafe) => Err(BosonIdError::UnsafeJobId),
    }
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` run ids
/// before run detail lookups.
///
/// # Errors
///
/// Returns a [`BosonIdError`] variant when the id fails the same rules as
/// [`validate_task_name`].
pub fn validate_run_id(run_id: &str) -> Result<(), BosonIdError> {
    match check_ops_id(run_id) {
        Ok(_) => Ok(()),
        Err(BosonIdErrorKind::Empty) => Err(BosonIdError::EmptyRunId),
        Err(BosonIdErrorKind::TooLong) => Err(BosonIdError::RunIdTooLong),
        Err(BosonIdErrorKind::Unsafe) => Err(BosonIdError::UnsafeRunId),
    }
}

const HEX: &[u8; 16] = b"0123456789ABCDEF";

/// Percent-encode a single path (or query-value) segment for `/boson/...` hrefs.
///
/// Leaves RFC 3986 unreserved characters alone (`ALPHA` / `DIGIT` / `-` `.` `_`
/// `~`). Encodes `/`, `\`, controls, spaces, and other bytes so Orbital
/// `paths::*` format strings cannot smuggle extra path segments.
#[must_use]
pub fn encode_ops_path_segment(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for &b in raw.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(b as char);
            }
            _ => {
                out.push('%');
                out.push(HEX[(b >> 4) as usize] as char);
                out.push(HEX[(b & 0x0f) as usize] as char);
            }
        }
    }
    out
}

/// `/boson/tasks/{encoded}` detail href.
#[must_use]
pub fn boson_task_path(task_name: &str) -> String {
    format!("/boson/tasks/{}", encode_ops_path_segment(task_name))
}

/// `/boson/tasks/{encoded}/config` href.
#[must_use]
pub fn boson_task_config_path(task_name: &str) -> String {
    format!("/boson/tasks/{}/config", encode_ops_path_segment(task_name))
}

/// `/boson/runs/{encoded}` detail href.
#[must_use]
pub fn boson_run_path(run_id: &str) -> String {
    format!("/boson/runs/{}", encode_ops_path_segment(run_id))
}

/// `/boson/runs?job={encoded}` filter href.
#[must_use]
pub fn boson_runs_job_filter_path(job_id: &str) -> String {
    format!("/boson/runs?job={}", encode_ops_path_segment(job_id))
}

/// Dashboard chart window: last 24 hours (seconds).
pub const RANGE_SECS_24H: i64 = 86_400;
/// Dashboard chart window: last 7 days (seconds).
pub const RANGE_SECS_7D: i64 = 604_800;

/// Inclusive priority bounds accepted by the ops UI config update.
pub const MIN_TASK_PRIORITY: i32 = -1_000;
/// Inclusive priority bounds accepted by the ops UI config update.
pub const MAX_TASK_PRIORITY: i32 = 10_000;
/// Maximum retry attempts (including the first try) accepted from the UI.
pub const MAX_RETRY_ATTEMPTS: u32 = 100;
/// Maximum backoff delay (ms) accepted from the UI.
pub const MAX_RETRY_DELAY_MS: u64 = 86_400_000;
/// Maximum Unicode scalar count for a pool name in a config update.
pub const MAX_POOL_NAME_CHARS: usize = 128;

/// Invalid dashboard range or task-config update field rejected at the server boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum BosonInputError {
    /// `range_secs` was not an allowed dashboard window.
    InvalidRangeSecs,
    /// Priority was outside [`MIN_TASK_PRIORITY`]..=[`MAX_TASK_PRIORITY`].
    PriorityOutOfRange,
    /// Pool string was blank, oversized, or path-unsafe.
    InvalidPool,
    /// Retry `max_attempts` was zero or above [`MAX_RETRY_ATTEMPTS`].
    InvalidMaxAttempts,
    /// Retry delay fields exceeded [`MAX_RETRY_DELAY_MS`].
    InvalidRetryDelay,
    /// Retry backoff multiplier was non-finite or not positive.
    InvalidBackoffMultiplier,
}

impl std::fmt::Display for BosonInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRangeSecs => write!(
                f,
                "Invalid range_secs: expected {RANGE_SECS_24H} (24h) or {RANGE_SECS_7D} (7d)"
            ),
            Self::PriorityOutOfRange => write!(
                f,
                "Invalid task config update: priority must be between {MIN_TASK_PRIORITY} and {MAX_TASK_PRIORITY}"
            ),
            Self::InvalidPool => write!(
                f,
                "Invalid task config update: pool must be a non-empty path-safe name"
            ),
            Self::InvalidMaxAttempts => write!(
                f,
                "Invalid task config update: max_attempts must be between 1 and {MAX_RETRY_ATTEMPTS}"
            ),
            Self::InvalidRetryDelay => write!(
                f,
                "Invalid task config update: retry delays must be at most {MAX_RETRY_DELAY_MS} ms"
            ),
            Self::InvalidBackoffMultiplier => write!(
                f,
                "Invalid task config update: backoff_multiplier must be a finite positive number"
            ),
        }
    }
}

impl std::error::Error for BosonInputError {}

/// Accepts only the dashboard chart windows the UI exposes (24h / 7d).
///
/// # Errors
///
/// Returns [`BosonInputError::InvalidRangeSecs`] when `range_secs` is not
/// [`RANGE_SECS_24H`] or [`RANGE_SECS_7D`].
pub const fn validate_range_secs(range_secs: i64) -> Result<(), BosonInputError> {
    if range_secs == RANGE_SECS_24H || range_secs == RANGE_SECS_7D {
        Ok(())
    } else {
        Err(BosonInputError::InvalidRangeSecs)
    }
}

/// Validates optional fields on a partial task-config update before apply/upsert.
///
/// Unset (`None`) fields are skipped. Set fields must stay within ops UI bounds.
///
/// # Errors
///
/// Returns a [`BosonInputError`] variant when priority, pool, or retry policy
/// fields are out of range or path-unsafe.
pub fn validate_task_config_update(
    req: &crate::types::UpdateTaskConfigRequest,
) -> Result<(), BosonInputError> {
    if let Some(priority) = req.priority {
        if !(MIN_TASK_PRIORITY..=MAX_TASK_PRIORITY).contains(&priority) {
            return Err(BosonInputError::PriorityOutOfRange);
        }
    }
    if let Some(ref pool) = req.pool {
        let trimmed = pool.trim();
        if trimmed.is_empty()
            || trimmed.chars().count() > MAX_POOL_NAME_CHARS
            || trimmed == "."
            || trimmed == ".."
            || trimmed.chars().any(is_unsafe_ops_id_char)
        {
            return Err(BosonInputError::InvalidPool);
        }
    }
    if let Some(ref retry) = req.retry_policy {
        if retry.max_attempts == 0 || retry.max_attempts > MAX_RETRY_ATTEMPTS {
            return Err(BosonInputError::InvalidMaxAttempts);
        }
        if retry.base_delay_ms > MAX_RETRY_DELAY_MS || retry.max_delay_ms > MAX_RETRY_DELAY_MS {
            return Err(BosonInputError::InvalidRetryDelay);
        }
        if !retry.backoff_multiplier.is_finite() || retry.backoff_multiplier <= 0.0 {
            return Err(BosonInputError::InvalidBackoffMultiplier);
        }
    }
    Ok(())
}
