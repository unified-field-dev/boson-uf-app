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

Crate-root rustdoc owns Organized-by-task, Features, the route table, and the
Examples. Pure mapping and id validation live in `boson-backend`. CI Layer 1
(SSR clippy/check) and Layer 2 (Playwright) gates are listed in
`docs/VERIFICATION.md` at the workspace root.

Compose into a host that supplies a Boson coordinator and the auth/context
extractors the app expects. Enable `ssr` / `hydrate` to match your host. For
Help spotlight tours, enable `uf-integrations` `offering-help` (or `full`) and
call `boson_app::ensure_help_steps_linked()`. The `e2e-lab` feature is for the
Playwright lab host only — do not enable it on product hosts.
