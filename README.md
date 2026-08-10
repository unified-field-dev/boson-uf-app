# Boson UF App

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[GitHub](https://github.com/deathbreakfast/boson-uf-app) · `cargo doc -p boson-backend --open` · distributed via git (not crates.io)

## About

Boson UF App is the Unified Field **operations UI** for Boson queues, task
config, and run history under `/boson`. Boson itself has no built-in UI; hosts
mount this crate so operators can inspect and operate background work.

- **UI (`boson-app`)** — pages, Higgs `#[server]` wrappers, `BosonRoutes`,
  `uf_app!` registration
- **Backend (`boson-backend`)** — pure job/run/task/dashboard helpers (no Leptos);
  preferred Layer 1 CI path

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
boson-app = { git = "https://github.com/deathbreakfast/boson-uf-app", package = "boson-app", rev = "REPLACE_WITH_PIN", default-features = false }
boson-backend = { git = "https://github.com/deathbreakfast/boson-uf-app", package = "boson-backend", rev = "REPLACE_WITH_PIN" }
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

Top-level `uf-*` directories in this checkout (if present) are unused leftovers.
Real `uf-integrations` / `uf-product-macros` / `uf-ssr` / `uf-app-registry` pins
live in workspace `[workspace.dependencies]` (see `Cargo.toml`).

## Examples

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`protected-boson-host`](examples/protected-boson-host/) | Auth + `/boson` dashboard API | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-boson-uf-app cargo run -p protected-boson-host` | Deny/allow + KPI JSON | Product host with `BosonRoutes` |

Full ladder: [`examples/README.md`](examples/README.md).

| Level | Where |
|-------|--------|
| Highlight | Mount snippet above; crate-root Getting started |
| Mid | `boson-backend` unit + integ suites (see `docs/VERIFICATION.md`) |
| Detailed | `protected-boson-host` (session gate + dashboard KPIs) |

## Security

Auth-gated `/boson` routes (task config also requires a verified email) and
private vulnerability reporting: [`SECURITY.md`](SECURITY.md). Report
vulnerabilities privately — do not open a public issue for security-sensitive
reports.

## Verify

Local gates (fmt/clippy/CI workflow not claimed here):

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-boson-uf-app
cargo clippy -p boson-backend --all-targets -- -D warnings
cargo test -p boson-backend
RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc -p boson-backend --no-deps
```

Prefer `boson-backend` for contract CI. `boson-app` compile/doc can fail when
the path-patched Orbital / host graph is broken upstream — treat that as
host-product debt, not a Boson mapping gap. Full command block:
[`docs/VERIFICATION.md`](docs/VERIFICATION.md). Contribute:
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
