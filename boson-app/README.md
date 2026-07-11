# boson-app

**Zone B** — official Orbital UI for Boson queue inspection, runs, task config, and dashboards.

**Tracker:** `boson_extract_01` · [`boson/EXTRACTION.md`](../boson/EXTRACTION.md)

## Role

Product-facing admin UI mounted at `/boson` in the Unified Field template. Depends on:

- [`boson`](../boson/) facade (enqueue/admin API via server functions)
- Orbital components (data tables, charts, layout)
- Optional Photon live updates via [`boson-photon-events`](../boson-photon-events/README.md)

Unified Field keeps a thin wrapper only where permissions, Gluon pool labels, or UF-specific routes differ.

## E2E

- **Official app flows:** boson-app integration tests (Phase 5+) — queue, runs, task-config
- **Template shell:** [`end2end/tests/boson/boson.spec.ts`](../end2end/tests/boson/boson.spec.ts) — nav, auth, root visibility only

## Must not

- Reference Zone A internal module paths; use public boson API and Zone B adapters only

## Status

Implemented — dashboard, queue, runs, tasks, task config pages. See [`BOSON_UI_AUDIT.md`](BOSON_UI_AUDIT.md).
