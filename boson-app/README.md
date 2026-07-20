# boson-app

Leptos admin UI for Boson: queue inspection, runs, task config, and dashboards (typically mounted at `/boson`).

Depends on the Boson facade (enqueue/admin APIs via server functions), Orbital UI components, and optionally Photon for live updates.

## Pages

Dashboard, queue, runs, tasks, and task-config views. See [`BOSON_UI_AUDIT.md`](BOSON_UI_AUDIT.md) for UI coverage notes.

## Feature checks

```bash
cargo check -p boson-app
cargo check -p boson-app --features hydrate
cargo check -p boson-app --features ssr
```
