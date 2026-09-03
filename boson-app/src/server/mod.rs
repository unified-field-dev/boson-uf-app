//! Leptos server functions and DTOs for Boson UI.
//!
//! DTOs and pure mapping helpers live in [`boson_backend`] so contracts stay
//! unit/integration-testable without the host UI graph. Server functions run on
//! SSR only and use [`higgs::Higgs::from_request()`] plus [`helpers::require_session`]
//! on every endpoint. Mutators `cancel_job` / `update_task_config` and task-config
//! reads (`get_task_config`, `list_gluon_pools_for_boson_task_config`) require
//! Gauge permission `BosonAdmin`. Task-config endpoints additionally mirror the
//! UI email-verification gate via [`helpers::require_email_verified`].
//!
//! The UI uses paginated list endpoints (`get_tasks_page`, `list_jobs_page`,
//! `list_runs_page`) and single-record getters (`get_task`, `get_run`, etc.).
//!
//! ## Errors
//!
//! Fallible ops return [`ServerFnError`](leptos::prelude::ServerFnError) (Leptos
//! boundary). Stable message prefixes integrators can match:
//!
//! - Auth: `Authentication is required…`, `Email verification is required…`
//! - Context: `Boson backend not in request context`
//! - Id validation: [`boson_backend::BosonIdError`] Display text
//! - Range: `Invalid range_secs:…` ([`boson_backend::BosonInputError`])
//! - Config update: `Invalid task config update:…`
//! - Config load: `Failed to load task config:…` / `Task config not found:…`
//! - Mutators: `Failed to cancel job:…` / `Failed to update config:…`
//!
//! Blank / oversized / path-unsafe ids are rejected by `boson_backend::validate_*`
//! before coordinator IO. Detail hrefs use `boson_backend::boson_*_path` helpers so
//! Orbital `paths::*` format strings cannot smuggle extra path segments. Failures
//! are traced once at the server-fn boundary (`operation`, `error_class`, safe ids).

mod dashboard;
mod gluon_pools;
mod helpers;
mod jobs;
pub mod page_query;
mod runs;
mod tasks;
mod types;

pub use types::*;

/// Permission name required for Boson admin mutators / task-config reads
/// (manifest: [`crate::permissions::BosonPermission::BosonAdmin`]).
pub const BOSON_ADMIN_PERMISSION: &str = "BosonAdmin";

pub use dashboard::{get_dashboard_stats, get_run_stats_series};
pub use gluon_pools::list_gluon_pools_for_boson_task_config;
pub use jobs::{cancel_job, list_jobs_datatable_page, list_jobs_page};
pub use runs::{get_run, list_runs_datatable_page, list_runs_page};
pub use tasks::{
    get_task, get_task_config, get_tasks, get_tasks_datatable_page, get_tasks_page,
    update_task_config,
};
