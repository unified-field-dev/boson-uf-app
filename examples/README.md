# Examples

Runnable teaching hosts for this UF app. Each card: when to use · command ·
success · look next.

## Canonical path

### `protected-boson-host` — auth + `/boson` dashboard

**Teaches:** session auth gate on `/boson` and the in-memory dashboard KPI shape
`boson-backend` builds for the UI. Inventory names: `boson` / `/boson` /
`RequireAuthenticated` / `BosonAdmin`.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-boson-uf-app
cargo run -p protected-boson-host
```

**Success:** stdout prints `protected_boson_host: OK — /boson deny/allow + dashboard KPIs`.

**Next step:** Mount `<BosonRoutes />` in a product host with Higgs + Boson
coordinator.

Copy table + product mount `Cargo.toml`:
[`protected-boson-host/README.md`](protected-boson-host/README.md).

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`protected-boson-host`](protected-boson-host/) | Auth + `/boson` dashboard API | `cargo run -p protected-boson-host` | Deny/allow + KPI JSON | Product host with `BosonRoutes` |
