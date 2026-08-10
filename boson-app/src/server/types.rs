//! Re-export UI-facing DTOs from [`boson_backend`].

pub use boson_backend::{
    clamp_page_list_limit, DashboardChartPoint, DashboardChartSeries, DashboardStats,
    GluonPoolPickRow, JobStatusDto, JobSummary, RetryPolicyDto, RunStatusDto, RunSummary,
    TaskConfigDto, TaskSummary, UpdateTaskConfigRequest, BOSON_LIST_FETCH_CAP, JOBS_PAGE_SIZE,
    MAX_PAGE_LIST_LIMIT, RUNS_PAGE_SIZE, TASKS_PAGE_SIZE,
};
