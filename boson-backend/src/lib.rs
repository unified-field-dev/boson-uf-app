//! Pure backend contracts for the Boson UF app server surface.
//!
//! Leptos `#[server]` entrypoints in `boson-app` resolve Higgs / coordinator
//! request context, then call these helpers so job, run, task, and dashboard
//! shapes stay unit- and integration-testable without a full host or UI graph.
//!
//! ## Organized by task
//!
//! | Task | Start here |
//! |------|------------|
//! | **Validate list/detail ids** | [`validate_task_name`], [`validate_job_id`], [`validate_run_id`] |
//! | **Task list/detail mapping** | [`TaskSummary`], [`find_task_by_name`], [`sort_tasks_by_name`], [`filter_tasks_by_query`] |
//! | **Job / queue mapping** | [`JobSummary`], [`find_job_by_id`], [`job_to_summary`], [`parse_job_status_filter`] |
//! | **Run list/detail mapping** | [`RunSummary`], [`find_run_by_id`], [`run_to_summary`] |
//! | **Task config update** | [`TaskConfigDto`], [`apply_task_config_update`], [`task_config_to_dto`] |
//! | **Dashboard KPIs / trends** | [`DashboardStats`], [`dashboard_stats`], [`run_stats_series_from_runs`] |
//! | **`DataTable` query adapters** | [`apply_jobs_datatable_query`], [`apply_runs_datatable_query`] |
//! | **UI pages / `#[server]` wrappers** | `boson-app` (not this crate) |
//!
//! ## Owns / does not own
//!
//! **Owns:** DTO shapes and pure mapping/validation helpers used by the Boson
//! ops UI server surface.
//!
//! **Does not own:** Leptos pages, Higgs `#[server]` wrappers, or route registration
//! (`boson-app`); Boson coordinator execution or `IsolatedLab` persistence (Boson core).
//!
//! ## Concern → API
//!
//! | Concern | API | Owner |
//! |---------|-----|-------|
//! | Id / name validation | [`validate_task_name`], [`validate_job_id`], [`validate_run_id`] | this crate |
//! | Task summaries / filters | [`TaskSummary`], [`find_task_by_name`], [`sort_tasks_by_name`], [`filter_tasks_by_query`] | this crate |
//! | Job summaries / status filter | [`JobSummary`], [`find_job_by_id`], [`job_to_summary`], [`parse_job_status_filter`] | this crate |
//! | Run summaries | [`RunSummary`], [`find_run_by_id`], [`run_to_summary`] | this crate |
//! | Task config DTOs | [`TaskConfigDto`], [`apply_task_config_update`], [`task_config_to_dto`] | this crate |
//! | Dashboard aggregates | [`DashboardStats`], [`dashboard_stats`], [`run_stats_series_from_runs`] | this crate |
//! | `DataTable` adapters | [`apply_jobs_datatable_query`], [`apply_runs_datatable_query`] | this crate |
//! | Pages, routes, server fns | `boson-app` (`BosonRoutes`) | `boson-app` |
//!
//! ## Examples ladder
//!
//! | Level | Where |
//! |-------|--------|
//! | Highlight | Concern → API table above |
//! | Mid | This crate's unit + integ suites (`docs/VERIFICATION.md`) |
//! | Detailed | `examples/protected-boson-host` |

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod dashboard;
mod lookup;
mod map;
mod page_query;
mod types;
mod validate;

pub use dashboard::{
    align_run_bucket, fill_bucket_range, run_bucket_granularity, run_stats_series_from_runs,
    RunBucketGranularity,
};
pub use lookup::{
    filter_tasks_by_query, find_job_by_id, find_run_by_id, find_task_by_name, sort_tasks_by_name,
};
pub use map::{
    aggregate_task_stats, apply_task_config_update, dashboard_stats, default_gluon_pool_rows,
    job_status_to_dto, job_to_summary, parse_job_status_filter, retry_policy_from_dto,
    retry_policy_to_dto, run_status_to_dto, run_to_summary, success_rate_pct, task_config_to_dto,
    task_summary_from_parts, TaskStatsAgg,
};
pub use page_query::{
    apply_jobs_datatable_query, apply_runs_datatable_query, extract_status_filter, job_status_key,
    quick_search_text, resolve_job_filter, run_status_key,
};
pub use types::{
    clamp_page_list_limit, DashboardChartPoint, DashboardChartSeries, DashboardStats,
    GluonPoolPickRow, JobStatusDto, JobSummary, RetryPolicyDto, RunStatusDto, RunSummary,
    TaskConfigDto, TaskSummary, UpdateTaskConfigRequest, BOSON_LIST_FETCH_CAP, JOBS_PAGE_SIZE,
    MAX_PAGE_LIST_LIMIT, RUNS_PAGE_SIZE, TASKS_PAGE_SIZE,
};
pub use validate::{validate_job_id, validate_run_id, validate_task_name};

#[cfg(test)]
mod tests {
    use boson_core::{Job, JobStatus, RetryPolicy, Run, RunStatus, TaskConfig};
    use chrono::{TimeZone, Timelike, Utc};
    use orbital_data::DataValue;
    use orbital_paging::{FilterLogicWire, FilterQuery, FilterRuleParam, PageRequest};

    use super::*;

    fn sample_job_core(id: &str, task: &str, status: JobStatus) -> Job {
        Job {
            job_id: id.into(),
            task_name: task.into(),
            actor_json: serde_json::json!({}),
            params_json: serde_json::json!({}),
            priority: 3,
            pool: "global".into(),
            status,
            idempotency_key: None,
            created_at: Utc::now(),
            signature_hash: 0,
            attempt: 1,
        }
    }

    fn sample_run_core(id: &str, job: &str, task: &str, status: RunStatus) -> Run {
        let now = Utc::now();
        Run {
            run_id: id.into(),
            job_id: job.into(),
            task_name: task.into(),
            attempt: 1,
            status,
            started_at: now,
            finished_at: Some(now),
            duration_ms: Some(10),
            error_message: None,
        }
    }

    fn sample_task(name: &str) -> TaskSummary {
        TaskSummary {
            name: name.into(),
            signature_json: "{}".into(),
            default_priority: 0,
            default_pool: "global".into(),
            effective_priority: 0,
            effective_pool: "global".into(),
            jobs_queued: 0,
            runs_total: 0,
            success_rate_pct: None,
        }
    }

    #[test]
    fn validate_task_name_accepts_non_empty_happy_path() {
        validate_task_name("alpha").expect("non-empty");
        validate_task_name("  beta  ").expect("trimmed non-empty");
    }

    #[test]
    fn validate_task_name_rejects_blank_sad() {
        let err = validate_task_name("").expect_err("blank");
        assert!(err.contains("required"), "{err}");
        let err = validate_task_name("   ").expect_err("whitespace");
        assert!(err.contains("required"), "{err}");
    }

    #[test]
    fn validate_job_id_rejects_blank_sad() {
        let err = validate_job_id("").expect_err("blank");
        assert!(err.contains("required"), "{err}");
    }

    #[test]
    fn validate_run_id_rejects_blank_sad() {
        let err = validate_run_id("  ").expect_err("whitespace");
        assert!(err.contains("required"), "{err}");
    }

    #[test]
    fn validate_job_id_accepts_id_happy_path() {
        validate_job_id("job-1").expect("id");
    }

    #[test]
    fn validate_run_id_accepts_id_happy_path() {
        validate_run_id("run-1").expect("id");
    }

    #[test]
    fn find_task_by_name_resolves_exact_happy_path() {
        let tasks = vec![sample_task("alpha"), sample_task("beta")];
        let found = find_task_by_name(&tasks, "beta").expect("listed");
        assert_eq!(found.name, "beta");
    }

    #[test]
    fn find_task_by_name_unknown_is_none_sad() {
        let tasks = vec![sample_task("alpha")];
        assert!(find_task_by_name(&tasks, "__boson_missing_task__").is_none());
    }

    #[test]
    fn find_job_by_id_resolves_exact_happy_path() {
        let jobs = vec![job_to_summary(sample_job_core(
            "j1",
            "alpha",
            JobStatus::Queued,
        ))];
        let found = find_job_by_id(&jobs, "j1").expect("listed");
        assert_eq!(found.task_name, "alpha");
    }

    #[test]
    fn find_job_by_id_unknown_is_none_sad() {
        let jobs = vec![job_to_summary(sample_job_core(
            "j1",
            "alpha",
            JobStatus::Queued,
        ))];
        assert!(find_job_by_id(&jobs, "__boson_missing_job__").is_none());
    }

    #[test]
    fn find_run_by_id_resolves_exact_happy_path() {
        let runs = vec![run_to_summary(sample_run_core(
            "r1",
            "j1",
            "alpha",
            RunStatus::Success,
        ))];
        let found = find_run_by_id(&runs, "r1").expect("listed");
        assert_eq!(found.job_id, "j1");
    }

    #[test]
    fn find_run_by_id_unknown_is_none_sad() {
        let runs = vec![run_to_summary(sample_run_core(
            "r1",
            "j1",
            "alpha",
            RunStatus::Success,
        ))];
        assert!(find_run_by_id(&runs, "__boson_missing_run__").is_none());
    }

    #[test]
    fn sort_tasks_by_name_orders_lexicographically_happy_path() {
        let mut tasks = vec![sample_task("zeta"), sample_task("alpha")];
        sort_tasks_by_name(&mut tasks);
        assert_eq!(tasks[0].name, "alpha");
        assert_eq!(tasks[1].name, "zeta");
    }

    #[test]
    fn filter_tasks_by_query_happy_path() {
        let mut tasks = vec![sample_task("alpha"), sample_task("beta")];
        filter_tasks_by_query(&mut tasks, Some("alp"));
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].name, "alpha");
    }

    #[test]
    fn filter_tasks_by_query_blank_keeps_all_sad() {
        let mut tasks = vec![sample_task("alpha"), sample_task("beta")];
        filter_tasks_by_query(&mut tasks, Some("   "));
        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn parse_job_status_filter_maps_known_happy_path() {
        assert_eq!(parse_job_status_filter("queued"), Some(JobStatus::Queued));
        assert_eq!(parse_job_status_filter("failed"), Some(JobStatus::Failed));
    }

    #[test]
    fn parse_job_status_filter_unknown_is_none_sad() {
        assert_eq!(parse_job_status_filter("unknown"), None);
        assert_eq!(parse_job_status_filter("QUEUED"), None);
        assert_eq!(parse_job_status_filter(""), None);
    }

    #[test]
    fn job_and_run_to_summary_preserve_identity_happy_path() {
        let summary = job_to_summary(sample_job_core("j1", "alpha", JobStatus::Queued));
        assert_eq!(summary.job_id, "j1");
        assert_eq!(summary.status, JobStatusDto::Queued);
        assert_eq!(summary.priority, 3);

        let mut run = sample_run_core("r1", "j1", "alpha", RunStatus::Failed);
        run.error_message = Some("boom".into());
        run.duration_ms = Some(42);
        let run_summary = run_to_summary(run);
        assert_eq!(run_summary.run_id, "r1");
        assert_eq!(run_summary.status, RunStatusDto::Failed);
        assert_eq!(run_summary.duration_ms, Some(42));
        assert_eq!(run_summary.error_message.as_deref(), Some("boom"));
    }

    #[test]
    fn aggregate_task_stats_groups_by_task_name_happy_path() {
        let jobs = vec![
            sample_job_core("j1", "alpha", JobStatus::Queued),
            sample_job_core("j2", "alpha", JobStatus::Running),
        ];
        let runs = vec![
            sample_run_core("r1", "j1", "alpha", RunStatus::Success),
            sample_run_core("r2", "j2", "beta", RunStatus::Failed),
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

    #[test]
    fn success_rate_pct_none_when_no_runs_sad() {
        assert_eq!(success_rate_pct(0, 0), None);
    }

    #[test]
    fn success_rate_pct_computes_percent_happy_path() {
        assert_eq!(success_rate_pct(4, 3), Some(75.0));
    }

    #[test]
    fn task_summary_from_parts_applies_config_happy_path() {
        let mut config = TaskConfig::default_for("alpha");
        config.priority = 9;
        config.pool = "urgent".into();
        config.retry_policy = RetryPolicy {
            max_attempts: 3,
            base_delay_ms: 100,
            backoff_multiplier: 2.0,
            max_delay_ms: 1000,
        };
        let summary = task_summary_from_parts(
            "alpha",
            r#"{"args":[]}"#,
            1,
            "global",
            &config,
            TaskStatsAgg {
                jobs_queued: 2,
                runs_total: 4,
                success_count: 2,
            },
        );
        assert_eq!(summary.effective_priority, 9);
        assert_eq!(summary.effective_pool, "urgent");
        assert_eq!(summary.success_rate_pct, Some(50.0));
    }

    #[test]
    fn apply_task_config_update_merges_fields_happy_path() {
        let mut config = TaskConfig::default_for("alpha");
        let req = UpdateTaskConfigRequest {
            priority: Some(5),
            pool: Some("batch".into()),
            retry_policy: Some(RetryPolicyDto {
                max_attempts: 7,
                base_delay_ms: 50,
                backoff_multiplier: 1.5,
                max_delay_ms: 500,
            }),
        };
        let now = Utc.with_ymd_and_hms(2026, 1, 2, 3, 4, 5).unwrap();
        apply_task_config_update(&mut config, &req, now);
        let dto = task_config_to_dto(&config);
        assert_eq!(dto.priority, 5);
        assert_eq!(dto.pool, "batch");
        assert_eq!(dto.retry_policy.max_attempts, 7);
        assert_eq!(dto.updated_at, now.to_rfc3339());
    }

    #[test]
    fn dashboard_stats_shape_happy_path() {
        let stats = dashboard_stats(3, 4, 1, 9);
        assert_eq!(stats.task_count, 3);
        assert_eq!(stats.jobs_queued, 4);
        assert_eq!(stats.jobs_running, 1);
        assert_eq!(stats.runs_today, 9);
    }

    #[test]
    fn run_bucket_granularity_switches_at_one_day_happy_path() {
        assert_eq!(
            run_bucket_granularity(86_400),
            RunBucketGranularity::FourHourly
        );
        assert_eq!(run_bucket_granularity(86_401), RunBucketGranularity::Daily);
    }

    #[test]
    fn align_run_bucket_four_hourly_floors_happy_path() {
        let ts = Utc.with_ymd_and_hms(2026, 1, 1, 10, 45, 0).unwrap();
        let aligned = align_run_bucket(ts, RunBucketGranularity::FourHourly);
        assert_eq!(aligned.hour(), 8);
        assert_eq!(aligned.minute(), 0);
    }

    #[test]
    fn run_stats_series_omits_failed_when_all_success_happy_path() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        let mut run = sample_run_core("r1", "j1", "alpha", RunStatus::Success);
        run.started_at = now - chrono::Duration::hours(1);
        let series = run_stats_series_from_runs(&[run], now, 86_400);
        assert_eq!(series.len(), 1);
        assert_eq!(series[0].id, "successful");
        assert!(series[0].points.iter().any(|p| p.value > 0.0));
    }

    #[test]
    fn extract_status_filter_reads_equals_happy_path() {
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: None,
            filter: Some(FilterQuery {
                logic: FilterLogicWire::And,
                items: vec![FilterRuleParam {
                    field: "status".into(),
                    operator: "equals".into(),
                    value: DataValue::Text("queued".into()),
                }],
            }),
            sort: None,
        };
        assert_eq!(extract_status_filter(&request), Some("queued".into()));
    }

    #[test]
    fn extract_status_filter_ignores_non_status_sad() {
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: None,
            filter: Some(FilterQuery {
                logic: FilterLogicWire::And,
                items: vec![FilterRuleParam {
                    field: "task_name".into(),
                    operator: "equals".into(),
                    value: DataValue::Text("alpha".into()),
                }],
            }),
            sort: None,
        };
        assert_eq!(extract_status_filter(&request), None);
    }

    #[test]
    fn resolve_job_filter_prefers_scope_happy_path() {
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: None,
            filter: Some(FilterQuery {
                logic: FilterLogicWire::And,
                items: vec![FilterRuleParam {
                    field: "job_id".into(),
                    operator: "equals".into(),
                    value: DataValue::Text("from-filter".into()),
                }],
            }),
            sort: None,
        };
        assert_eq!(
            resolve_job_filter(Some("from-scope".into()), &request),
            Some("from-scope".into())
        );
    }

    #[test]
    fn quick_search_text_rejects_blank_sad() {
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: Some("  ".into()),
            filter: None,
            sort: None,
        };
        assert_eq!(quick_search_text(&request), None);
    }

    #[test]
    fn apply_jobs_datatable_query_filters_quick_search_happy_path() {
        let mut jobs = vec![
            job_to_summary(sample_job_core("j1", "alpha", JobStatus::Queued)),
            job_to_summary(sample_job_core("j2", "beta", JobStatus::Running)),
        ];
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: Some("alpha".into()),
            filter: None,
            sort: None,
        };
        apply_jobs_datatable_query(&mut jobs, &request);
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].job_id, "j1");
    }

    #[test]
    fn apply_runs_datatable_query_status_equals_happy_path() {
        let mut runs = vec![
            run_to_summary(sample_run_core("r1", "j1", "alpha", RunStatus::Success)),
            run_to_summary(sample_run_core("r2", "j2", "beta", RunStatus::Failed)),
        ];
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: None,
            filter: Some(FilterQuery {
                logic: FilterLogicWire::And,
                items: vec![FilterRuleParam {
                    field: "status".into(),
                    operator: "equals".into(),
                    value: DataValue::Text("failed".into()),
                }],
            }),
            sort: None,
        };
        apply_runs_datatable_query(&mut runs, &request);
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].run_id, "r2");
    }

    #[test]
    fn default_gluon_pool_rows_includes_global_happy_path() {
        let rows = default_gluon_pool_rows();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, "global");
    }

    #[test]
    fn clamp_page_list_limit_caps_oversized_sad() {
        assert_eq!(clamp_page_list_limit(10_000), MAX_PAGE_LIST_LIMIT);
        assert_eq!(
            clamp_page_list_limit(MAX_PAGE_LIST_LIMIT),
            MAX_PAGE_LIST_LIMIT
        );
    }

    #[test]
    fn clamp_page_list_limit_preserves_small_happy_path() {
        assert_eq!(clamp_page_list_limit(20), 20);
        assert_eq!(clamp_page_list_limit(0), 0);
    }

    #[test]
    fn job_status_dto_serde_roundtrip_happy_path() {
        let status = JobStatusDto::Queued;
        let json = serde_json::to_string(&status).expect("serialize");
        assert_eq!(json, "\"queued\"");
        let back: JobStatusDto = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, JobStatusDto::Queued);
    }
}
