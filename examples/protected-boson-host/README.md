# protected-boson-host

Axum oneshot host under **`/boson`**: deny without session, allow with
`X-Demo-User`, return the in-memory dashboard KPI shape `boson-backend` builds
for the UI.

Production Leptos hosts mount `BosonRoutes` at **`/boson`** and gate mutating
ops with `BosonAdmin`. This example proves the same path + auth + dashboard
contract without the SSR/WASM / Orbital graph. The oneshot path `/boson`
matches the Orbital app id/path (`boson` / `/boson`).

| | |
|---|---|
| **When to use** | First smoke of Boson UF app host wiring (auth gate + dashboard API) |
| **Command** | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-boson-uf-app cargo run -p protected-boson-host` |
| **Success** | Stdout: `protected_boson_host: OK — /boson deny/allow + dashboard KPIs` |
| **Look next** | Mount [`BosonRoutes`](../../boson-app/) ; wire Higgs + Boson coordinator |

**Open first:** [`src/main.rs`](src/main.rs)

## Copy into your host

| File | What to take |
|------|----------------|
| This [`Cargo.toml`](Cargo.toml) | Axum oneshot shape + `boson-backend` (dashboard KPI smoke) |
| Product mount `Cargo.toml` (below) | `boson-app` + `boson-backend` with `ssr` / `hydrate` features |
| [`src/main.rs`](src/main.rs) | Session gate on `/boson`, dashboard JSON, inventory contract names |
| Leptos sketch (below) | `<BosonRoutes />` under `/boson` |

### Product mount dependencies

```toml
[dependencies]
boson-app = { git = "https://github.com/deathbreakfast/boson-uf-app", package = "boson-app", rev = "REPLACE_WITH_PIN", default-features = false }
boson-backend = { git = "https://github.com/deathbreakfast/boson-uf-app", package = "boson-backend", rev = "REPLACE_WITH_PIN" }
uf-product = { /* your pin */, default-features = false }
uf-integrations = { /* your pin */, default-features = false }

[features]
ssr = [
    "boson-app/ssr",
    "uf-product/ssr",
    "uf-integrations/ssr",
]
hydrate = [
    "boson-app/hydrate",
    "uf-product/hydrate",
    "uf-integrations/hydrate",
]
```

### Leptos mount sketch

```rust,ignore
use boson_app::BosonRoutes;
use leptos_router::components::Routes;

view! {
    <Routes fallback=|| "not found">
        <BosonRoutes />
    </Routes>
}
```

Dashboard helpers (Leptos-free):

```rust,ignore
use boson_backend::dashboard_stats;

let stats = dashboard_stats(task_count, jobs_queued, jobs_running, runs_today);
```

Inventory names match `boson` / `/boson`. Layout uses `RequireAuthenticated`;
ops mutators carry `BosonAdmin` (manifest
`permissions::BosonPermission`). Task config also requires a verified email.
Wire Higgs + Boson coordinator + session extractors in host bootstrap before
mounting the routes.

For shell chrome (layout, fonts, Axum + Leptos boot), copy
[`shell-chrome-host`](https://github.com/deathbreakfast/unified-field-product/tree/main/examples/shell-chrome-host)
from unified-field-product, then mount `BosonRoutes`.

## Run (documented gate)

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-boson-uf-app
cargo check -p protected-boson-host
cargo run -p protected-boson-host
```

**Success:** stdout prints `protected_boson_host: OK — /boson deny/allow + dashboard KPIs`.

## Hydrate / browser

Out of gate for this host. Full ops UI needs a product binary with
`cargo-leptos`, `wasm32`, session chrome, Higgs + Boson coordinator, and a
working Orbital / `uf-product` graph. Prefer the oneshot above for local gates;
treat `boson-app` compile failures from broken sibling pins as host-product debt.
