# Boson UF App

[![CI](https://github.com/unified-field-dev/boson-uf-app/actions/workflows/ci.yml/badge.svg)](https://github.com/unified-field-dev/boson-uf-app/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[GitHub](https://github.com/unified-field-dev/boson-uf-app) · `cargo doc -p boson-backend --open`

## About

Boson UF App is the Unified Field **operations UI** for Boson queues, task
config, and run history under `/boson`. Boson itself has no built-in UI; hosts
mount this crate so operators can inspect and operate background work.

- **UI (`boson-app`)** — pages, Higgs `#[server]` wrappers, `BosonRoutes`,
  `uf_app!` registration
- **Backend (`boson-backend`)** — pure job/run/task/dashboard helpers (no Leptos);
  primary CI surface

Hosts supply a Boson coordinator and auth guard context. Enable `ssr` / hydrate
to match your host. Crate-root rustdoc owns Concern → route → server fn tables;
prefer `cargo doc -p boson-backend --open` for the mapping contract. UI rustdoc
is pin-dependent on Orbital / host graphs. Poll-based refresh is live today;
`photon_ws` / `live` are the stubbed integration points for Photon push once a
host wires them.

## Getting started

```toml
[dependencies]
# Pin tag or rev — do not use branch = "main".
boson-app = { git = "https://github.com/unified-field-dev/boson-uf-app", package = "boson-app", rev = "REPLACE_WITH_PIN", default-features = false }
boson-backend = { git = "https://github.com/unified-field-dev/boson-uf-app", package = "boson-backend", rev = "REPLACE_WITH_PIN" }
```

```rust,ignore
use boson_app::BosonRoutes;
use leptos_router::components::Routes;

view! {
    <Routes fallback=|| "not found">
        <BosonRoutes />
    </Routes>
}
```

Wire Boson coordinator + session extractors in host bootstrap, then mount the
routes above. Full Leptos SSR hosts live outside this repository; use the local
teaching host for the auth + dashboard contract.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-boson-uf-app
cargo test -p boson-backend
```

## Workspace

| Crate | Role |
|-------|------|
| [`boson-app`](boson-app/) | Leptos ops UI + `BosonRoutes` + app registration |
| [`boson-backend`](boson-backend/) | Pure DTO/mapping helpers for job/run/task/dashboard |
| [`protected-boson-host`](examples/protected-boson-host/) | Teaching host: deny/allow + dashboard KPIs |

## Examples

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`protected-boson-host`](examples/protected-boson-host/) | Auth + `/boson` dashboard API | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-boson-uf-app cargo run -p protected-boson-host` | Deny/allow + KPI JSON | Mount `BosonRoutes` |

Copy table + product mount `Cargo.toml`:
[`examples/protected-boson-host/README.md`](examples/protected-boson-host/README.md).
Full ladder: [`examples/README.md`](examples/README.md).

| Level | Where |
|-------|--------|
| Highlight | Mount snippet above; crate-root Getting started |
| Mid | `boson-backend` unit + integ suites (see `docs/VERIFICATION.md`) |
| Detailed | `protected-boson-host` (session gate + dashboard KPIs; inventory `boson` / `/boson`) |

## Security

Auth-gated `/boson` routes (task config also requires a verified email) and
private vulnerability reporting: [`SECURITY.md`](SECURITY.md). Report
vulnerabilities privately — do not open a public issue for security-sensitive
reports.

## Verify

GitHub Actions (`.github/workflows/ci.yml`) runs the CI subset from
[`docs/VERIFICATION.md`](docs/VERIFICATION.md): fmt, clippy `-D warnings` on
`boson-backend` (+ teaching host), contract tests, `protected-boson-host`
check/run, and boson-backend rustdoc with broken-intra-doc-link deny.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-boson-uf-app
cargo fmt -p boson-backend -p boson-app -p protected-boson-host -- --check
cargo clippy -p boson-backend --all-targets -- -D warnings
cargo clippy -p protected-boson-host --all-targets -- -D warnings
cargo test -p boson-backend --test workspace_members --test product_surface
cargo test -p boson-backend
cargo check -p protected-boson-host
cargo run -p protected-boson-host
RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc -p boson-backend --no-deps
```

Teaching host success line:
`protected_boson_host: OK — /boson deny/allow + dashboard KPIs`.
Full command block: [`docs/VERIFICATION.md`](docs/VERIFICATION.md). Contribute:
[`CONTRIBUTING.md`](CONTRIBUTING.md).

## FAQ

**Is this a standalone Boson server?** No. `boson-app` mounts under a host
`<Routes>` tree. Job execution and persistence live in the Boson coordinator /
core crates.

**Why is there a separate `boson-backend` crate?** So job/run/task and dashboard
helpers stay unit-testable without the Leptos/UI dependency graph. `boson-app`
`#[server]` fns are thin wrappers over those helpers.

**What can operators change from the UI?** Task configuration (priority, pools,
retry) and cancelling queued jobs. List/detail and dashboard views are read
paths. Task config additionally requires a verified email.

**Where does Boson core fit?** Enqueue, run, and IsolatedLab contracts live in
the Boson coordinator / core repos. This repo maps admin/list/get/update APIs
into UF ops pages.

## License

MIT. See [LICENSE](LICENSE), [CONTRIBUTING.md](CONTRIBUTING.md),
[SECURITY.md](SECURITY.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
