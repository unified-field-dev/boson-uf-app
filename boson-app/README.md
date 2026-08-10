# boson-app

Leptos operations UI for Boson: queues, runs, task config, and dashboards under
`/boson`.

```toml
# Pin tag or rev — do not use branch = "main".
boson-app = { git = "https://github.com/unified-field-dev/boson-uf-app", package = "boson-app", rev = "REPLACE_WITH_PIN", default-features = false }
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

Crate-root rustdoc owns Organized-by-task, Owns / does not own, the route table,
and the Examples ladder. Mapping helpers live in `boson-backend`.

Compose into a host that supplies a Boson coordinator and the auth/context
extractors the app expects. Enable `ssr` / hydrate to match your host.
