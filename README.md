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
- Queue and run history views (including live updates when Photon WS is wired)

Host must supply a Boson backend and auth guard context expected by the app. Enable `ssr` / hydrate features to match your host.

## Workspace

| Crate | Role |
|-------|------|
| `boson-app` | Boson admin UI |
| `uf-*` | Thin shell / registry helpers shared with other uf-app repos |

## Verify

```bash
export CARGO_BUILD_JOBS=1
cargo check --workspace
cargo check -p boson-app --features ssr
cargo test -p boson-app --features ssr
```

## License

MIT. See [LICENSE](LICENSE), [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
