# boson-uf-app verification

Re-run after code or doc changes. This workspace is the L2 Boson operations app
(`boson-app` Leptos UI + `boson-backend` pure server contracts). Layer 1 unit +
integration tests cover job/run/task/dashboard helpers backing the `#[server]`
surface. No Leptos UI e2e, `*-e2e` crate, or AWS campaign is required for this
L2 app. Boson coordinator / IsolatedLab contracts own persistence and execution;
this repo verifies the UF app mapping layer.

## Environment

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-boson-uf-app
```

## Layer 1 — Unit + integration (CI)

Backend contracts (preferred path; no UI graph):

```bash
cargo fmt --all --check
cargo clippy -p boson-backend --all-targets -- -D warnings
cargo test -p boson-backend
```

Full workspace (includes `boson-app` UI). May fail when the path-patched
`uf-product` / `uf-integrations` UI graph is broken upstream — that is a
pre-existing host-product UI compile issue, not a Boson backend contract gap:

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
# Host-aligned SSR surface (when UI graph compiles):
cargo test -p boson-app --features ssr
```

### TEST_MAP

| Behavior | Level | Happy | Sad | Notes |
|----------|-------|-------|-----|-------|
| `validate_task_name` | unit+integ | non-empty / trimmed name | blank / whitespace → `"required"` | gate for get_task / config |
| `validate_job_id` | unit+integ | non-empty id | blank → `"required"` | gate for cancel_job |
| `validate_run_id` | unit+integ | non-empty id | blank → `"required"` | gate for get_run |
| `find_task_by_name` (`get_task`) | unit+integ | exact name → summary | unknown → `None` | list/detail contract |
| `find_job_by_id` (`cancel_job` / queue) | unit+integ | exact id → summary | unknown → `None` | queue contract |
| `find_run_by_id` (`get_run`) | unit+integ | exact id → summary | unknown → `None` | run detail contract |
| `sort_tasks_by_name` / `filter_tasks_by_query` | unit+integ | lex order / substring match | blank query keeps all; unknown → `[]` | tasks page |
| `parse_job_status_filter` | unit+integ | known lowercase statuses | unknown / wrong-case → `None` | jobs list |
| `job_to_summary` / `run_to_summary` / aggregates | unit+integ | identity + queued/success counts | no runs → `success_rate_pct = None` | get_tasks |
| `apply_task_config_update` / `task_config_to_dto` | unit+integ | partial merge | — | update_task_config |
| `apply_*_datatable_query` / filters | unit+integ | search + status equals/OR | non-status → no status filter; blank search → `None` | DataTable adapters |
| `dashboard_stats` / `run_stats_series_from_runs` | unit+integ | KPI shape / success+failed series | outside window → zero buckets | dashboard |
| Higgs `#[server]` fns + session / `BosonAdmin` / email-verified gates | — | — | — | deferred — needs host SSR |
| Leptos UI / Playwright / `cargo leptos` e2e | e2e | — | — | **waived** — covering integ named below |
| IsolatedLab job/run/task e2e | e2e | — | — | **waived** — covered by boson coordinator + Layer 1 integ |
| AWS / soak | AWS | — | — | **waived** — L2 app; no cloud resources |
| Micro-benchmarks | bench | — | — | **waived** — no hot-path campaign |

## Layer 2 — E2E

**Waived.** Task/job/run list+detail, status filters, DataTable query adapters,
dashboard KPI/trend shapes, and id/name validation are exercised by Layer 1
integration tests named below. A Leptos/UI browser suite or IsolatedLab `*-e2e`
crate is out of scope for this backend-first L2 remediation; live Boson
execution/persistence IsolatedLab belongs in boson coordinator / core.

Covering integ tests for the e2e waiver:

- `get_tasks_list_sorted_and_named_happy_path` / `get_task_detail_matches_list_entry_happy_path` / `get_task_unknown_name_is_none_sad`
- `get_run_detail_matches_list_entry_happy_path` / `get_run_unknown_id_is_none_sad`
- `cancel_job_list_entry_resolves_happy_path` / `cancel_job_unknown_id_is_none_sad`
- `tasks_page_filters_by_query_happy_path` / `tasks_page_filters_unknown_query_empty_sad`
- `list_jobs_status_filter_parses_known_happy_path` / `list_jobs_status_filter_unknown_is_none_sad`
- `get_tasks_aggregates_stats_happy_path` / `update_task_config_merges_partial_happy_path`
- `validate_*_accepts_*_happy_path` / `validate_*_rejects_blank_sad`
- `jobs_datatable_*` / `runs_datatable_*` / `extract_status_filter_*` / `resolve_job_filter_*`
- `dashboard_stats_aggregates_counts_happy_path` / `run_stats_series_24h_includes_success_and_failed_happy_path` / `run_stats_series_all_outside_window_zero_success_sad`

## Layer 3 — AWS campaigns + performance

**Waived.** L2 application workspace; no cloud resources or Criterion benches.
Correctness is in-process against Boson UF app DTO/mapping contracts only.

## Notes

- Prefer `cargo test -p boson-backend` for backend contract CI when the UI
  dependency graph (`uf-product` via `uf-integrations` / `lepton-shell`) fails to
  compile — report that separately from Boson contract results.
- Tests may `unwrap`/`expect`; production server fns map failures to `ServerFnError`
  (no ordinary-path unwrap).
- Sad-path assertions check message content or `None` / empty — (stronger than `is_err()` alone).
- Happy-path tests are named `*_happy_path` so audits detect them.
- `BosonRoutes` data loaders call the `#[server]` fns; those fns are thin Higgs
  wrappers over the helpers listed in the TEST_MAP.
