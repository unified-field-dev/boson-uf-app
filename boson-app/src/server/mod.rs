//! Leptos server functions and DTOs for Boson UI.
//!
//! DTOs and pure mapping helpers live in [`boson_backend`] so contracts stay
//! unit/integration-testable without the host UI graph. Server functions run on
//! SSR only and use [`higgs::Higgs::from_request()`] plus [`helpers::require_session`]
//! on every endpoint. Mutators `cancel_job` / `update_task_config` also require
//! Gauge permission `BosonAdmin`. Task-config reads/writes additionally mirror the
//! UI email-verification gate via [`helpers::require_email_verified`].
//!
//! The UI uses paginated list endpoints (`get_tasks_page`, `list_jobs_page`,
//! `list_runs_page`) and single-record getters (`get_task`, `get_run`, etc.).

mod dashboard;
mod gluon_pools;
mod helpers;
mod jobs;
pub mod page_query;
mod runs;
mod tasks;
mod types;

pub use types::*;

pub use dashboard::{get_dashboard_stats, get_run_stats_series};
pub use gluon_pools::list_gluon_pools_for_boson_task_config;
pub use jobs::{cancel_job, list_jobs_datatable_page, list_jobs_page};
pub use runs::{get_run, list_runs_datatable_page, list_runs_page};
pub use tasks::{
    get_task, get_task_config, get_tasks, get_tasks_datatable_page, get_tasks_page,
    update_task_config,
};
