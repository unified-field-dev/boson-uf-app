# Boson UF App

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Leptos admin UI for Boson queues, task config, and run history — mounted under `/boson`.

```toml
[dependencies]
boson-app = { git = "https://github.com/deathbreakfast/boson-uf-app", package = "boson-app", branch = "main" }
```

```rust
use boson_app::BosonRoutes;
use leptos_router::components::Routes;

view! {
    <Routes fallback=|| "not found">
        <BosonRoutes />
    </Routes>
}
```

## About

- Dashboard for aggregate task/job/run activity
- Task configuration (priority, pools, retry)
- Queue and run history views (poll-based refresh today; `photon_ws`/`live` are the
  stubbed integration points for real Photon push updates once a host wires them)

Host must supply a Boson backend and auth guard context expected by the app. Enable `ssr` / hydrate features to match your host. See the `boson-app` crate rustdocs for the full Concern → route → server fn table.

## Examples

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`protected-boson-host`](examples/protected-boson-host/) | Auth + `/boson` dashboard API | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-boson-uf-app cargo run -p protected-boson-host` | Deny/allow + KPI JSON | Product host with `BosonRoutes` |

Full ladder: [`examples/README.md`](examples/README.md).

## Workspace

| Crate | Role |
|-------|------|
| `boson-app` | Boson admin UI |
| `uf-*` (top-level `uf-app-registry`, `uf-integrations`, `uf-product-macros`, `uf-ssr`) | Not workspace members and not depended on — the workspace's real `uf-*` crates come from `L3-zone-products` (see `[workspace.dependencies]` in `Cargo.toml`). These local trees are unused leftovers; do not treat them as source of truth. |

## Verify

```bash
export CARGO_BUILD_JOBS=1
cargo check --workspace
cargo check -p boson-app --features ssr
cargo test -p boson-app --features ssr
```

## License

MIT. See [LICENSE](LICENSE), [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
