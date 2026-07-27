# protected-boson-host

Axum oneshot host under **`/boson`**: deny without session, allow with `X-Demo-User`, return the in-memory dashboard KPI shape `boson-backend` builds for the UI.

Production Leptos hosts mount `<BosonRoutes />` (which wraps pages in `RequireAuthenticated`). This example proves the same path + auth + dashboard contract without the full SSR/WASM graph.

| | |
|---|---|
| **When to use** | First smoke of Boson UF app host wiring (auth gate + dashboard API) |
| **Command** | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-boson-uf-app cargo run -p protected-boson-host` |
| **Success** | Stdout: `protected_boson_host: OK — /boson deny/allow + dashboard KPIs` |
| **Look next** | Mount [`BosonRoutes`](../../boson-app/) in a product host; wire Higgs + Boson coordinator |

**Open first:** [`src/main.rs`](src/main.rs)

Compile-check:

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-boson-uf-app
cargo check -p protected-boson-host
```
