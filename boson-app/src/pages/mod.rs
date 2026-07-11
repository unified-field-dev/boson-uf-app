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
