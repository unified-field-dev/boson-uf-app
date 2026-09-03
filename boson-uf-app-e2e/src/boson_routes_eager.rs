//! Eager `/boson` routes for the Playwright host.
//!
//! Production [`boson_app::BosonRoutes`] wraps leaf pages in `Lazy` for
//! wasm-split. Nested `Lazy` under `ParentRoute` still panics on
//! `hydrate_body` in this Leptos pin, so the lab host mounts the same page
//! components without `Lazy`.

use boson_app::{
    BosonLayout, BosonQueuePage, BosonRootPage, BosonRunDetailPage, BosonRunsIndexPage,
    BosonTaskConfigPage, BosonTaskDetailPage, BosonTasksIndexPage,
};
use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route},
    path,
};
use uf_product::routes::RequireAuthenticated;

/// Same paths as [`boson_app::BosonRoutes`], without Lazy route views.
#[component(transparent)]
pub fn BosonRoutesEager() -> impl leptos_router::MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("boson") view=BosonLayout>
            <Route path=path!("") view=BosonRootPage />
            <Route path=path!("tasks") view=BosonTasksIndexPage />
            <Route path=path!("tasks/:task_name") view=BosonTaskDetailPage />
            <Route path=path!("tasks/:task_name/config") view=BosonVerifiedTaskConfigEager />
            <Route path=path!("queue") view=BosonQueuePage />
            <Route path=path!("runs") view=BosonRunsIndexPage />
            <Route path=path!("runs/:id") view=BosonRunDetailPage />
        </ParentRoute>
    }
    .into_inner()
}

#[component]
fn BosonVerifiedTaskConfigEager() -> impl IntoView {
    view! {
        <RequireAuthenticated requires_email_verification=true>
            <BosonTaskConfigPage />
        </RequireAuthenticated>
    }
}
