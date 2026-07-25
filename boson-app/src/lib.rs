#![recursion_limit = "256"]
//! Boson operations app: routes and UI composition for monitoring and operating Boson
//! background work queues under `/boson`.
//!
//! Boson itself is a background-job execution crate with no built-in UI; this crate is the
//! `#[uf_product_macros::orbital_app]`-registered operations surface a host mounts to give
//! operators visibility into (and control over) task configuration, queued jobs, and run
//! history.
//!
//! ## Features
//!
//! - **Dashboard** — [`BosonRootPage`] shows aggregate task/job/run activity and run trends.
//! - **Tasks** — [`BosonTasksIndexPage`] / [`BosonTaskDetailPage`] /
//!   [`BosonTaskConfigPage`] for browsing and editing task configuration (priority, pools,
//!   retry policy).
//! - **Queue** — [`BosonQueuePage`] for inspecting pending/active jobs.
//! - **Runs** — [`BosonRunsIndexPage`] / [`BosonRunDetailPage`] for historical run
//!   inspection and troubleshooting, including live updates via [`photon_ws`].
//!
//! ## Getting started
//!
//! Mount [`BosonRoutes`] inside your host's `<Routes>`; it registers the `/boson` subtree
//! (auth-gated) and, via `orbital_app!`, its launcher metadata:
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
//! ## Where to look next
//!
//! - [`BosonRoutes`] — the route entrypoint mounted by hosts.
//! - [`BosonLayout`] — the shared app bar / nav shell wrapping every route.
//! - [`pages`] — the page components listed under Features above.

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
use uf_product_macros::orbital_app;

mod components;
mod layout;
mod lazy_routes;
mod live;
pub mod pages;
#[cfg(feature = "ssr")]
pub mod photon_ws;
mod server;

pub use layout::BosonLayout;
pub use lazy_routes::{
    prefetch_family, BosonLayoutRouteView, BosonQueueRoute, BosonRootRoute, BosonRunDetailRoute,
    BosonRunsIndexRoute, BosonTaskDetailRoute, BosonTasksIndexRoute, BosonVerifiedTaskConfigRoute,
};
pub use pages::{
    BosonQueuePage, BosonRootPage, BosonRunDetailPage, BosonRunsIndexPage, BosonTaskConfigPage,
    BosonTaskDetailPage, BosonTasksIndexPage,
};

orbital_app! {
    name: "Boson",
    id: "boson",
    description: "Background work management",
    icon: "⚛️",
    version: "0.1.0",
    routes: BosonRoutes,
    route_path: "/boson",
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
