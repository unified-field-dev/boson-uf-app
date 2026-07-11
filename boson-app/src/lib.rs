#![recursion_limit = "256"]
//! Boson operations app routes and UI composition.
//!
//! This app provides the UI for monitoring and operating Boson background work
//! queues under `/boson`.
//!
//! ## UI features
//!
//! - Dashboard stats for task/job/run activity.
//! - Task index/detail/configuration views.
//! - Queue/job list and filtering views.
//! - Run history and run detail views.
//!
//! ## What it manages
//!
//! - Task configuration (priority, pools, retry policy).
//! - Job queue lifecycle actions.
//! - Historical run inspection and troubleshooting.
//!
//! ## Backend API surface
//!
//! The app's server module provides dashboard, task, queue/job, and run APIs in
//! [`server`].
//!
//! Route entrypoint: [`BosonRoutes`].

use leptos::prelude::*;
use leptos_router::{components::*, path};
use uf_product_macros::orbital_app;

mod components;
mod layout;
mod live;
mod pages;
#[cfg(feature = "ssr")]
pub mod photon_ws;
mod server;

pub use layout::BosonLayout;
pub use pages::{
    BosonQueuePage, BosonRootPage, BosonRunDetailPage, BosonRunsIndexPage, BosonTaskConfigPage,
    BosonTaskDetailPage, BosonTasksIndexPage,
};

#[component]
fn BosonAuthGuard() -> impl IntoView {
    view! {
        <orbital::routes::RequireAuthenticated>
            <BosonLayout />
        </orbital::routes::RequireAuthenticated>
    }
}

#[component]
fn BosonVerifiedTaskConfigPage() -> impl IntoView {
    view! {
        <orbital::routes::RequireAuthenticated requires_email_verification=true>
            <BosonTaskConfigPage />
        </orbital::routes::RequireAuthenticated>
    }
}

orbital_app! {
    name: "Boson",
    id: "boson",
    description: "Background work management",
    icon: "⚛️",
    version: "0.1.0",
    routes: BosonRoutes,
    route_path: "/boson",
}

#[orbital_macros::orbital_routes_extract]
#[component(transparent)]
pub fn BosonRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("boson") view=BosonAuthGuard>
            <Route path=path!("") view=BosonRootPage />
            <Route path=path!("tasks") view=BosonTasksIndexPage />
            <Route path=path!("tasks/:task_name") view=BosonTaskDetailPage />
            <Route path=path!("tasks/:task_name/config") view=BosonVerifiedTaskConfigPage />
            <Route path=path!("queue") view=BosonQueuePage />
            <Route path=path!("runs") view=BosonRunsIndexPage />
            <Route path=path!("runs/:id") view=BosonRunDetailPage />
        </ParentRoute>
    }
    .into_inner()
}
