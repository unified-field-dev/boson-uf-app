# boson-uf-app-e2e

Leptos lab host + Playwright for [`boson-app`](../boson-app/) `BosonRoutes`.

Mounts the same pages a product host would under `/boson`, with lab-only mem
Valence, session injection, and an in-process MemQueue Boson coordinator.
**Do not copy this boot into a production host.**

## Run

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-boson-uf-app
# From the boson-uf-app workspace root.
cargo leptos end-to-end --project boson-uf-app-e2e
```

Do not interrupt the end-to-end run. It stops on its own when Playwright finishes.

Site: `http://127.0.0.1:3170` · seed: `POST /api/test/seed-data`

## Scenarios

| ID | Asserts |
|----|---------|
| `pw-boson-auth-gate-*` | Anon gated; admin sees dashboard |
| `pw-boson-dashboard-*` | KPI / seeded task visible |
| `pw-boson-tasks-*` | List→detail; unknown task empty |
| `pw-boson-queue-cancel-*` | Admin cancel; non-admin denied |
| `pw-boson-task-config-*` | Unverified blocked; admin save |
| `pw-boson-runs-*` | List→detail; unknown run |
