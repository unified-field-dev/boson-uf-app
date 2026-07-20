# Boson UF App

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Official Unified Field admin UI for Boson (Leptos).

```toml
[dependencies]
boson-app = { git = "https://github.com/deathbreakfast/boson-uf-app", package = "boson-app", branch = "main" }
```

Mount Boson admin routes (queue, runs, task config, dashboards) from your host shell.

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
