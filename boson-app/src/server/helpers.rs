//! DTO mappers, auth helpers, and task summary builders for Boson server functions.

#[cfg(feature = "ssr")]
use std::collections::HashMap;

#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;

#[cfg(feature = "ssr")]
use super::types::{
    JobStatusDto, JobSummary, RetryPolicyDto, RunStatusDto, RunSummary, TaskSummary,
};

/// Per-task aggregates built from full job/run lists (`get_tasks`).
#[cfg(feature = "ssr")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TaskStatsAgg {
    pub jobs_queued: u32,
    pub runs_total: u32,
    pub success_count: u32,
}

#[cfg(feature = "ssr")]
pub(super) fn job_status_to_dto(s: boson_core::JobStatus) -> JobStatusDto {
    use boson_core::JobStatus;
    match s {
        JobStatus::Queued => JobStatusDto::Queued,
        JobStatus::Running => JobStatusDto::Running,
        JobStatus::Success => JobStatusDto::Success,
        JobStatus::Failed => JobStatusDto::Failed,
        JobStatus::Canceled => JobStatusDto::Canceled,
    }
}

#[cfg(feature = "ssr")]
pub(super) fn run_status_to_dto(s: boson_core::RunStatus) -> RunStatusDto {
    use boson_core::RunStatus;
    match s {
        RunStatus::Running => RunStatusDto::Running,
        RunStatus::Success => RunStatusDto::Success,
        RunStatus::Failed => RunStatusDto::Failed,
        RunStatus::Canceled => RunStatusDto::Canceled,
        RunStatus::Timeout => RunStatusDto::Timeout,
    }
}

#[cfg(feature = "ssr")]
pub(super) fn retry_policy_to_dto(r: &boson_core::RetryPolicy) -> RetryPolicyDto {
    RetryPolicyDto {
        max_attempts: r.max_attempts,
        base_delay_ms: r.base_delay_ms,
        backoff_multiplier: r.backoff_multiplier,
        max_delay_ms: r.max_delay_ms,
    }
}

#[cfg(feature = "ssr")]
pub(super) fn boson_backend(
) -> Result<std::sync::Arc<dyn boson_coordinator::BosonCoordinatorBackend>, ServerFnError> {
    leptos::context::use_context::<std::sync::Arc<dyn boson_coordinator::BosonCoordinatorBackend>>()
        .ok_or_else(|| ServerFnError::new("Boson backend not in request context"))
}

#[cfg(feature = "ssr")]
pub(super) fn ensure_verified_user(ctx: &higgs::Higgs) -> Result<(), ServerFnError> {
    if ctx.session_user_id().is_some() {
        Ok(())
    } else {
        Err(ServerFnError::new(
            "Authentication is required for this action",
        ))
    }
}

#[cfg(feature = "ssr")]
pub(super) fn parse_job_status_filter(s: &str) -> Option<boson_core::JobStatus> {
    use boson_core::JobStatus;
    match s {
        "queued" => Some(JobStatus::Queued),
        "running" => Some(JobStatus::Running),
        "success" => Some(JobStatus::Success),
        "failed" => Some(JobStatus::Failed),
        "canceled" => Some(JobStatus::Canceled),
        _ => None,
    }
}

#[cfg(feature = "ssr")]
pub(super) fn job_to_summary(j: boson_core::Job) -> JobSummary {
    JobSummary {
        job_id: j.job_id,
        task_name: j.task_name,
        status: job_status_to_dto(j.status),
        priority: j.priority,
        pool: j.pool,
        created_at: j.created_at.to_rfc3339(),
    }
}

#[cfg(feature = "ssr")]
pub(super) fn run_to_summary(r: boson_core::Run) -> RunSummary {
    RunSummary {
        run_id: r.run_id,
        job_id: r.job_id,
        task_name: r.task_name,
        status: run_status_to_dto(r.status),
        attempt: r.attempt,
        started_at: r.started_at.to_rfc3339(),
        finished_at: r.finished_at.map(|dt| dt.to_rfc3339()),
        duration_ms: r.duration_ms,
        error_message: r.error_message,
    }
}

#[cfg(feature = "ssr")]
fn success_rate_pct(runs_total: u32, success_count: u32) -> Option<f64> {
    if runs_total > 0 {
        Some((success_count as f64 / runs_total as f64) * 100.0)
    } else {
        None
    }
}

#[cfg(feature = "ssr")]
pub(super) fn task_summary_from_parts(
    desc: &boson_runtime::TaskDescriptor,
    config: &boson_core::TaskConfig,
    stats: TaskStatsAgg,
) -> TaskSummary {
    TaskSummary {
        name: desc.name.to_string(),
        signature_json: desc.signature_json.to_string(),
        default_priority: desc.default_priority,
        default_pool: desc.default_pool.to_string(),
        effective_priority: config.priority,
        effective_pool: config.pool.clone(),
        jobs_queued: stats.jobs_queued,
        runs_total: stats.runs_total,
        success_rate_pct: success_rate_pct(stats.runs_total, stats.success_count),
    }
}

#[cfg(feature = "ssr")]
pub(super) fn aggregate_task_stats(
    jobs: &[boson_core::Job],
    runs: &[boson_core::Run],
) -> HashMap<String, TaskStatsAgg> {
    use boson_core::{JobStatus, RunStatus};

    let mut out: HashMap<String, TaskStatsAgg> = HashMap::new();

    for job in jobs {
        let entry = out.entry(job.task_name.clone()).or_default();
        if job.status == JobStatus::Queued {
            entry.jobs_queued += 1;
        }
    }

    for run in runs {
        let entry = out.entry(run.task_name.clone()).or_default();
        entry.runs_total += 1;
        if run.status == RunStatus::Success {
            entry.success_count += 1;
        }
    }

    out
}

/// Build a single task summary via registry descriptor and backend stats (O(1) registry + task-scoped reads).
#[cfg(feature = "ssr")]
pub(super) async fn build_task_summary(
    backend: &dyn boson_coordinator::BosonCoordinatorBackend,
    desc: &boson_runtime::TaskDescriptor,
) -> Result<TaskSummary, ServerFnError> {
    use boson_core::{JobStatus, TaskConfig};

    let name = desc.name.to_string();
    let config = backend
        .get_task_config(&name)
        .await
        .unwrap_or_else(|_| TaskConfig::default_for(&name));

    let jobs_queued = backend
        .count_jobs_for_task(&name, Some(JobStatus::Queued))
        .await as u32;
    let run_stats = backend.task_run_stats(&name).await;

    let stats = TaskStatsAgg {
        jobs_queued,
        runs_total: run_stats.runs_total,
        success_count: run_stats.success_count,
    };

    Ok(task_summary_from_parts(desc, &config, stats))
}

#[cfg(test)]
mod tests {
    use orbital_paging::Page;

    use crate::server::types::TaskSummary;

    #[test]
    fn page_from_oversized_detects_has_more() {
        let items: Vec<TaskSummary> = (0..3)
            .map(|i| TaskSummary {
                name: format!("task-{i}"),
                signature_json: "{}".into(),
                default_priority: 0,
                default_pool: "global".into(),
                effective_priority: 0,
                effective_pool: "global".into(),
                jobs_queued: 0,
                runs_total: 0,
                success_rate_pct: None,
            })
            .collect();
        let page = Page::from_oversized(items, 2, Some(3));
        assert!(page.has_more);
        assert_eq!(page.items.len(), 2);
        assert_eq!(page.total_count, Some(3));
    }

    #[test]
    fn page_from_oversized_no_extra_page() {
        let items: Vec<TaskSummary> = (0..2)
            .map(|i| TaskSummary {
                name: format!("task-{i}"),
                signature_json: "{}".into(),
                default_priority: 0,
                default_pool: "global".into(),
                effective_priority: 0,
                effective_pool: "global".into(),
                jobs_queued: 0,
                runs_total: 0,
                success_rate_pct: None,
            })
            .collect();
        let page = Page::from_oversized(items, 2, Some(2));
        assert!(!page.has_more);
        assert_eq!(page.items.len(), 2);
    }

    #[cfg(feature = "ssr")]
    mod ssr {
        use boson_core::{Job, JobStatus, Run, RunStatus};

        use super::super::{
            aggregate_task_stats, job_status_to_dto, parse_job_status_filter, retry_policy_to_dto,
            run_status_to_dto, TaskStatsAgg,
        };
        use crate::server::types::{JobStatusDto, RunStatusDto};

        #[test]
        fn job_status_mapper_covers_all_variants() {
            assert_eq!(job_status_to_dto(JobStatus::Queued), JobStatusDto::Queued);
            assert_eq!(
                job_status_to_dto(JobStatus::Running),
                JobStatusDto::Running
            );
            assert_eq!(
                job_status_to_dto(JobStatus::Success),
                JobStatusDto::Success
            );
            assert_eq!(job_status_to_dto(JobStatus::Failed), JobStatusDto::Failed);
            assert_eq!(
                job_status_to_dto(JobStatus::Canceled),
                JobStatusDto::Canceled
            );
        }

        #[test]
        fn run_status_mapper_covers_all_variants() {
            assert_eq!(
                run_status_to_dto(RunStatus::Running),
                RunStatusDto::Running
            );
            assert_eq!(
                run_status_to_dto(RunStatus::Success),
                RunStatusDto::Success
            );
            assert_eq!(run_status_to_dto(RunStatus::Failed), RunStatusDto::Failed);
            assert_eq!(
                run_status_to_dto(RunStatus::Canceled),
                RunStatusDto::Canceled
            );
            assert_eq!(
                run_status_to_dto(RunStatus::Timeout),
                RunStatusDto::Timeout
            );
        }

        #[test]
        fn retry_policy_to_dto_preserves_fields() {
            let policy = boson_core::RetryPolicy {
                max_attempts: 5,
                base_delay_ms: 100,
                backoff_multiplier: 2.0,
                max_delay_ms: 30_000,
            };
            let dto = retry_policy_to_dto(&policy);
            assert_eq!(dto.max_attempts, 5);
            assert_eq!(dto.base_delay_ms, 100);
            assert_eq!(dto.backoff_multiplier, 2.0);
            assert_eq!(dto.max_delay_ms, 30_000);
        }

        #[test]
        fn parse_job_status_filter_maps_known_strings() {
            assert_eq!(
                parse_job_status_filter("queued"),
                Some(JobStatus::Queued)
            );
            assert_eq!(parse_job_status_filter("unknown"), None);
        }

        #[test]
        fn aggregate_task_stats_groups_by_task_name() {
            let now = chrono::Utc::now();
            let jobs = vec![
                Job {
                    job_id: "j1".into(),
                    task_name: "alpha".into(),
                    actor_json: serde_json::json!({}),
                    params_json: serde_json::json!({}),
                    priority: 0,
                    pool: "global".into(),
                    status: JobStatus::Queued,
                    idempotency_key: None,
                    created_at: now,
                    signature_hash: 0,
                    attempt: 1,
                },
                Job {
                    job_id: "j2".into(),
                    task_name: "alpha".into(),
                    actor_json: serde_json::json!({}),
                    params_json: serde_json::json!({}),
                    priority: 0,
                    pool: "global".into(),
                    status: JobStatus::Running,
                    idempotency_key: None,
                    created_at: now,
                    signature_hash: 0,
                    attempt: 1,
                },
            ];
            let runs = vec![
                Run {
                    run_id: "r1".into(),
                    job_id: "j1".into(),
                    task_name: "alpha".into(),
                    attempt: 1,
                    status: RunStatus::Success,
                    started_at: now,
                    finished_at: Some(now),
                    duration_ms: Some(10),
                    error_message: None,
                },
                Run {
                    run_id: "r2".into(),
                    job_id: "j2".into(),
                    task_name: "beta".into(),
                    attempt: 1,
                    status: RunStatus::Failed,
                    started_at: now,
                    finished_at: Some(now),
                    duration_ms: Some(20),
                    error_message: Some("err".into()),
                },
            ];

            let stats = aggregate_task_stats(&jobs, &runs);
            assert_eq!(
                stats.get("alpha"),
                Some(&TaskStatsAgg {
                    jobs_queued: 1,
                    runs_total: 1,
                    success_count: 1,
                })
            );
            assert_eq!(
                stats.get("beta"),
                Some(&TaskStatsAgg {
                    jobs_queued: 0,
                    runs_total: 1,
                    success_count: 0,
                })
            );
        }
    }
}
