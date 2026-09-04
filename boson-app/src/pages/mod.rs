//! Boson ops route pages mounted under `/boson`.
//!
//! Each page is a Leptos view composed with Orbital primitives. Server data comes from
//! [`crate::server`]. Prefer crate-root rustdoc for mount and server-fn teaching examples.

pub mod dashboard;
pub mod queue;
pub mod run_detail;
pub mod runs;
pub mod task_config;
pub mod task_detail;
pub mod tasks;

pub use dashboard::BosonRootPage;
pub use queue::BosonQueuePage;
pub use run_detail::BosonRunDetailPage;
pub use runs::BosonRunsIndexPage;
pub use task_config::BosonTaskConfigPage;
pub use task_detail::BosonTaskDetailPage;
pub use tasks::BosonTasksIndexPage;
