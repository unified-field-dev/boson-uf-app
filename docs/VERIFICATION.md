# boson-uf-app verification

Re-run after code or doc changes. This workspace is the Boson operations app
(`boson-app` Leptos UI + `boson-backend` pure server contracts). Layer 1 unit +
integration tests cover job/run/task/dashboard helpers backing the `#[server]`
surface, plus sibling-source UI surface contracts for `boson-app`. No Leptos UI
e2e, `*-e2e` crate, or AWS campaign is required for this workspace. Boson
coordinator / IsolatedLab contracts own persistence and execution; this repo
verifies the UF app mapping layer.

## Environment

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-boson-uf-app
```

## Teaching host (Pass 3 gate)

Axum oneshot under [`examples/protected-boson-host`](../examples/protected-boson-host/).
Copy table + product mount sketches live in that host README.

```bash
cargo check -p protected-boson-host
cargo run -p protected-boson-host
```

Success line: `protected_boson_host: OK — /boson deny/allow + dashboard KPIs`.
Hydrate/browser is out of gate for the oneshot (`cargo-leptos` + `wasm32` +
Orbital / `uf-product` belong to a composite product host).

## Layer 1 — Unit + integration (CI)

Sibling-source UI contracts (no Orbital / `boson-app` compile):

```bash
cargo test -p boson-backend --test workspace_members --test product_surface
```

Backend contracts (preferred path; no UI graph):

```bash
cargo fmt -p boson-backend -p boson-app -p protected-boson-host -- --check
cargo clippy -p boson-backend --all-targets -- -D warnings
cargo test -p boson-backend
```

`cargo fmt --all` can fail in this monorepo checkout when a path-patched
member sits outside that workspace; package-scoped fmt is the honest local gate.

Full workspace (includes `boson-app` UI). May fail when the path-patched
`uf-product` / `uf-integrations` UI graph is broken upstream — that is a
pre-existing host-product UI compile issue, not a Boson backend contract gap.
Surface needles for routes, nav testids, `RequireAuthenticated`, and
`BosonAdmin` live in `product_surface`.

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
# Host-aligned SSR surface (when UI graph compiles):
cargo test -p boson-app --features ssr
```

## Layer 2 — E2E

**Waived.** Task/job/run list+detail, status filters, DataTable query adapters,
dashboard KPI/trend shapes, and id/name validation are exercised by Layer 1
integration tests named below. A Leptos/UI browser suite or IsolatedLab `*-e2e`
crate is out of scope for this backend-first remediation; live Boson
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
- `boson_product_workspace_members_happy_path`
- `boson_routes_mount_happy_path` / `layout_auth_gate_and_nav_happy_path` / `admin_mutators_require_boson_admin_happy_path`

## Layer 3 — AWS campaigns + performance

**Waived.** This application workspace; no cloud resources or Criterion benches.
Correctness is in-process against Boson UF app DTO/mapping contracts only.

## Rustdoc policy

Preferred deny gate (no UI graph):

```bash
RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc -p boson-backend --no-deps
```

Workspace `rustdoc::broken_intra_doc_links` is `allow` in `Cargo.toml` because
sibling/cfg-gated links often fail under `--no-deps`. Prefer the
`RUSTDOCFLAGS` deny form above for the backend contract crate. `boson-app`
rustdoc with deny flags is pin-dependent on Orbital / host graphs.
`boson-app` still uses `#![allow(missing_docs)]` on macro-heavy UI surfaces.

## Notes

- Prefer `cargo test -p boson-backend` for backend contract CI when the UI
  dependency graph (`uf-product` via `uf-integrations` / `lepton-shell`) fails to
  compile — report that separately from Boson contract results.
- Tests may `unwrap`/`expect`; production server fns map failures to `ServerFnError`
  (no ordinary-path unwrap).
- Sad-path assertions check message content or `None` / empty — stronger than
  `is_err()` alone.
- Happy-path tests are named `*_happy_path` so audits detect them.
- `BosonRoutes` data loaders call the `#[server]` fns; those fns are thin Higgs
  wrappers over the helpers covered by Layer 1 contract tests.
