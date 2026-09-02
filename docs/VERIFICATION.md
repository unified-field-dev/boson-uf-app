# boson-uf-app verification

Re-run after code or doc changes. This workspace is the Boson operations app
(`boson-app` Leptos UI + `boson-backend` pure server contracts + `boson-uf-app-e2e`
lab host). Layer 1 unit + integration tests cover job/run/task/dashboard helpers
backing the `#[server]` surface, plus sibling-source UI surface contracts for
`boson-app`. Layer 2 is Playwright against a dedicated lab Leptos host that mounts
`BosonRoutes` with mem Valence and an in-process MemQueue coordinator. Boson
coordinator / core IsolatedLab contracts still own persistence and execution
matrix correctness.

## Environment

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-boson-uf-app
```

## Teaching host

Axum oneshot under [`examples/protected-boson-host`](../examples/protected-boson-host/).
Copy table + product mount sketches live in that host README.

```bash
cargo check -p protected-boson-host
cargo run -p protected-boson-host
```

Success line: `protected_boson_host: OK — /boson deny/allow + dashboard KPIs`.
Hydrate/browser is out of gate for the oneshot (`cargo-leptos` + `wasm32` +
Orbital / `uf-product` belong to a composite product host or `boson-uf-app-e2e`).

## Layer 1 — Unit + integration (CI)

GitHub Actions (`.github/workflows/ci.yml`) covers Layer 1 (backend contracts,
teaching host, **and** `boson-app` / `boson-uf-app-e2e` SSR check + clippy) plus
the boson-backend rustdoc gate and the Layer 2 Playwright job below.

Sibling-source UI contracts (no Orbital / `boson-app` compile):

```bash
cargo test -p boson-backend --test workspace_members --test product_surface
```

Backend + SSR surface (preferred CI path):

```bash
cargo fmt -p boson-backend -p boson-app -p protected-boson-host -p boson-uf-app-e2e -- --check
cargo clippy -p boson-backend --all-targets -- -D warnings
cargo clippy -p protected-boson-host --all-targets -- -D warnings
cargo clippy -p boson-app --features ssr --all-targets -- -D warnings
# boson-uf-app-e2e lab harness is expect-heavy; skip clippy (same as chronon-uf-app-e2e).
cargo test -p boson-backend
cargo check -p boson-app --features ssr
cargo check -p boson-uf-app-e2e --features ssr
```
`cargo fmt --all` can fail when a sibling checkout sits outside this workspace;
package-scoped fmt is the honest local gate.

SSR compile needs a green sibling `gauge` checkout (path-patched from
`L2-product-platform/gauge`). If `PermissionHistory` (or other gauge codegen) is
missing, fix gauge / `record-history` first — that is not a Boson backend
contract gap.

Full workspace (includes hydrate UI). May fail when the sibling
`uf-product` / `uf-integrations` UI graph does not compile — that is a
host-product UI issue, not a Boson backend contract gap.
Surface needles for routes, nav testids, `RequireAuthenticated`, and
`BosonAdmin` live in `product_surface` (structural secondary; Layer 2 is
primary for operator UI).

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test -p boson-app --features ssr
```

### leptos-lints (CI job `leptos-lints`)

Needs `cargo-dylint` / `dylint-link` 6.0.1 and toolchain `nightly-2025-05-14`
(see `leptos-lints@v0.1.2`). Workspace `[workspace.metadata.dylint]` pins the
library; rustc deny names are declared under `[workspace.lints.rust]`.
GitHub Actions runs the same command.

```bash
# cargo install cargo-dylint --locked --version 6.0.1
# cargo install dylint-link --locked --version 6.0.1
# rustup toolchain install nightly-2025-05-14 --component rustc-dev,llvm-tools-preview

export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-boson-uf-app
export CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=fallback
export RUSTFLAGS="-D warnings -Zcrate-attr=feature(stdarch_x86_avx512)"

cargo dylint --all -p boson-app --no-deps -- --features hydrate
```

Hard CI job for hydrate-only dylint remains deferred (Orbital / host pin risk).
Run locally when that graph is green.

## Layer 2 — E2E (lab host + Playwright)

Primary operator-UI gate. Dedicated lab host mounts eager `BosonRoutes` pages
(same components as production Lazy routes), mem Valence, Higgs session injection,
and MemQueue `CoordinatorAdapter`. Port `127.0.0.1:3170`. The lab enables
`boson-app/e2e-lab` for email-verification seed overrides.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-boson-uf-app
# From the boson-uf-app workspace root. Builds SSR + hydrate, then Playwright.
cargo leptos end-to-end --project boson-uf-app-e2e
```

Do not interrupt the end-to-end run. It stops when Playwright finishes.

Scenario IDs (validating happy + sad):

- `pw-boson-auth-gate-happy-admin` / `pw-boson-auth-gate-sad-anonymous`
- `pw-boson-dashboard-happy-kpis` / `pw-boson-dashboard-sad-empty-recent-not-crash`
- `pw-boson-tasks-happy-list-detail` / `pw-boson-tasks-sad-unknown-task`
- `pw-boson-queue-cancel-happy-admin` / `pw-boson-queue-cancel-sad-non-admin`
- `pw-boson-task-config-happy-admin-save` / `pw-boson-task-config-sad-unverified-email`
- `pw-boson-runs-happy-list-detail` / `pw-boson-runs-sad-unknown-run`

CI runs the same `cargo leptos end-to-end --project boson-uf-app-e2e` job on
every PR and push to main (lepton/neutrino parity). Workspace `Cargo.toml`
patches crates.io orbital onto uf-dev git so `ThemeInjection` is a single type.

Layer 1 helper contracts remain the mapping gate:

- `get_tasks_list_sorted_and_named_happy_path` / `get_task_unknown_name_is_none_sad`
- `get_run_detail_matches_list_entry_happy_path` / `get_run_unknown_id_is_none_sad`
- `cancel_job_list_entry_resolves_happy_path` / `cancel_job_unknown_id_is_none_sad`
- `tasks_page_filters_by_query_happy_path` / `tasks_page_filters_unknown_query_empty_sad`
- `dashboard_stats_aggregates_counts_happy_path` / `run_stats_series_all_outside_window_zero_success_sad`
- `validate_range_secs_*` / `validate_task_config_update_*` / `format_task_config_load_error_*`
- `boson_routes_mount_happy_path` / `layout_auth_gate_and_nav_happy_path` / `admin_mutators_require_boson_admin_happy_path`

## Layer 3 — Cloud + performance

**Waived.** This application workspace; no cloud resources or Criterion benches.
Correctness is in-process against Boson UF app DTO/mapping contracts and the
lab Playwright host. L0/L1 Boson execution campaigns stay in boson / coordinator.

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
  dependency graph (`uf-product` via `uf-integrations` / `lepton-shell` / `gauge`)
  fails to compile — report that separately from Boson contract results.
- Tests may `unwrap`/`expect`; production server fns map failures to `ServerFnError`
  (no ordinary-path unwrap). Task config load fails closed (`Failed to load task
  config:…`); dashboard `range_secs` and config updates are validated at the
  boundary.
- Sad-path assertions check message content or `None` / empty — stronger than
  `is_err()` alone.
- Happy-path tests are named `*_happy_path` so audits detect them.
- `BosonRoutes` data loaders call the `#[server]` fns; those fns are thin Higgs
  wrappers over the helpers covered by Layer 1 contract tests.
- `product_surface` sibling-source needles are secondary regression checks, not
  a substitute for Layer 2 Playwright.
- Product hosts must **not** enable `boson-app/e2e-lab` (lab email override).
