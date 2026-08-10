#![recursion_limit = "256"]
//! Boson operations app: routes and UI composition for monitoring and operating Boson
//! background work queues under `/boson`.
//!
//! Boson itself is a background-job execution crate with no built-in UI; this crate is the
//! `#[uf_product_macros::uf_app]`-registered operations surface a host mounts to give
//! operators visibility into (and control over) task configuration, queued jobs, and run
//! history.
//!
//! Orbital inventory macros (`uf_app!`, `orbital_routes_extract`) emit undocumented
//! associated items, so this crate allows `missing_docs` at the crate root while keeping
//! hand-written modules and items documented.
//!
//! ## Organized by task
//!
//! | Task | Start here |
//! |------|------------|
//! | **Mount `/boson` routes** | [`BosonRoutes`] |
//! | **Dashboard KPIs / run trends** | [`BosonRootPage`], [`mod@server`] |
//! | **Browse / edit tasks** | [`BosonTasksIndexPage`], [`BosonTaskDetailPage`], [`BosonTaskConfigPage`] |
//! | **Inspect / cancel queue jobs** | [`BosonQueuePage`] |
//! | **Browse run history** | [`BosonRunsIndexPage`], [`BosonRunDetailPage`] |
//! | **Poll / Photon live stubs** | [`live`], [`photon_ws`] |
//! | **Pure DTO / mapping helpers** | `boson-backend` (not this crate) |
//!
//! ## Owns / does not own
//!
//! **Owns:** Leptos pages, Higgs `#[server]` wrappers, layout/nav shell, permission
//! manifest, poll-tick / live stubs, and `uf_app!` / [`BosonRoutes`] registration.
//!
//! **Does not own:** Job/run/task/dashboard mapping helpers (`boson-backend`); Boson
//! coordinator execution or `IsolatedLab` persistence (Boson core); full Leptos SSR host
//! binaries (live outside this repository).
//!
//! ## Routes (Concern → page → server fn)
//!
//! Mounted under `/boson` by [`BosonRoutes`]. Task config requires a verified email.
//!
//! | Path | Page | Key server fn(s) |
//! |---|---|---|
//! | `/boson` | [`BosonRootPage`] | `get_dashboard_stats`, `get_tasks` |
//! | `/boson/tasks` | [`BosonTasksIndexPage`] | `get_tasks_page`, `get_tasks_datatable_page` |
//! | `/boson/tasks/:task_name` | [`BosonTaskDetailPage`] | `get_task` |
//! | `/boson/tasks/:task_name/config` (email-verified) | [`BosonTaskConfigPage`] | `get_task_config`, `update_task_config`, `list_gluon_pools_for_boson_task_config` |
//! | `/boson/queue` | [`BosonQueuePage`] | `list_jobs_page`, `list_jobs_datatable_page`, `cancel_job` |
//! | `/boson/runs` | [`BosonRunsIndexPage`] | `list_runs_page`, `list_runs_datatable_page` |
//! | `/boson/runs/:id` | [`BosonRunDetailPage`] | `get_run` |
//!
//! ## Getting started
//!
//! Mount [`BosonRoutes`] inside your host's `<Routes>`; it registers the `/boson` subtree
//! (auth-gated) and, via `uf_app!`, its launcher metadata:
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use leptos_router::components::Routes;
//! use boson_app::BosonRoutes;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <Routes fallback=|| "not found">
//!             <BosonRoutes />
//!         </Routes>
//!     }
//! }
//! ```
//!
//! ## Examples ladder
//!
//! | Level | Where |
//! |-------|--------|
//! | Highlight | Getting started above |
//! | Mid | `boson-backend` unit + integ suites (`docs/VERIFICATION.md`) |
//! | Detailed | `examples/protected-boson-host` (deny/allow + dashboard KPIs; inventory `boson` / `/boson`; copy README) |
//!
//! ## Where to look next
//!
//! - [`BosonRoutes`] — the route entrypoint mounted by hosts.
//! - [`BosonLayout`] — the shared app bar / nav shell wrapping every route.
//! - [`pages`] — the page components listed under Organized by task above.
//! - [`mod@server`] — server functions and DTOs backing the UI.
//! - [`live`] / [`photon_ws`] — client poll-tick and SSR route merge point for live
//!   updates (Photon push wiring is currently a stub; see module docs).

#![allow(missing_docs)]
#![allow(clippy::unused_unit, unused_imports)]
#![cfg_attr(
    feature = "ssr",
    allow(
        dead_code,
        unused_imports,
        unused_variables,
        unknown_lints,
        clippy::all,
    )
)]
use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route},
    path, Lazy,
};
use uf_product_macros::uf_app;

mod components;
mod layout;
mod lazy_routes;
/// Client-side live-update hooks (poll tick, placeholder broadcast sources).
pub mod live;
pub mod pages;
/// Permission manifest for Boson admin server functions.
pub mod permissions;
#[cfg(feature = "ssr")]
pub mod photon_ws;
/// SSR server functions and DTOs backing the Boson UI.
pub mod server;

pub use layout::BosonLayout;
pub use lazy_routes::{
    prefetch_family, BosonLayoutRouteView, BosonQueueRoute, BosonRootRoute, BosonRunDetailRoute,
    BosonRunsIndexRoute, BosonTaskDetailRoute, BosonTasksIndexRoute, BosonVerifiedTaskConfigRoute,
};
pub use pages::{
    BosonQueuePage, BosonRootPage, BosonRunDetailPage, BosonRunsIndexPage, BosonTaskConfigPage,
    BosonTaskDetailPage, BosonTasksIndexPage,
};

uf_app! {
    name: "Boson",
    id: "boson",
    description: "Background work management",
    icon: "⚛️",
    version: "0.1.0",
    routes: BosonRoutes,
    route_path: "/boson",
    permission_manifest: permissions::BosonPermission,
}

/// Boson's nested route tree, gated behind an auth guard and mounted at `/boson`.
///
/// Leaf pages are [`LazyRoute`](leptos_router::LazyRoute) views so
/// `cargo leptos --split` can emit a separate WASM chunk for this family.
/// Registers dashboard, task, queue, and run routes. The task config route additionally
/// requires a verified email. Intended to be used inside a host `<Routes>` component, e.g.
/// `<BosonRoutes />`.
#[allow(missing_docs)]
#[orbital_macros::orbital_routes_extract]
#[component(transparent)]
pub fn BosonRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("boson") view=BosonLayoutRouteView>
            <Route path=path!("") view={Lazy::<BosonRootRoute>::new()} />
            <Route path=path!("tasks") view={Lazy::<BosonTasksIndexRoute>::new()} />
            <Route path=path!("tasks/:task_name") view={Lazy::<BosonTaskDetailRoute>::new()} />
            <Route path=path!("tasks/:task_name/config") view={Lazy::<BosonVerifiedTaskConfigRoute>::new()} />
            <Route path=path!("queue") view={Lazy::<BosonQueueRoute>::new()} />
            <Route path=path!("runs") view={Lazy::<BosonRunsIndexRoute>::new()} />
            <Route path=path!("runs/:id") view={Lazy::<BosonRunDetailRoute>::new()} />
        </ParentRoute>
    }
    .into_inner()
}
