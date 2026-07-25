//! Lazy-loaded route views for WASM code-splitting (`cargo leptos --split`).

use leptos::prelude::*;
use leptos_router::{lazy_route, LazyRoute};

use crate::{
    BosonLayout, BosonQueuePage, BosonRootPage, BosonRunDetailPage, BosonRunsIndexPage,
    BosonTaskConfigPage, BosonTaskDetailPage, BosonTasksIndexPage,
};

/// Prefetch the boson family WASM chunk (leaf pages share split modules).
pub async fn prefetch_family() {
    BosonRootRoute::preload().await;
}

/// Eager layout shell for `/boson/*` ParentRoute (auth gate lives inside [`BosonLayout`]).
#[component]
pub fn BosonLayoutRouteView() -> impl IntoView {
    view! { <BosonLayout /> }
}

/// Lazy `/boson` dashboard.
#[derive(Clone, Copy, Debug, Default)]
pub struct BosonRootRoute;

#[lazy_route]
impl LazyRoute for BosonRootRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <BosonRootPage /> }.into_any()
    }
}

/// Lazy `/boson/tasks`.
#[derive(Clone, Copy, Debug, Default)]
pub struct BosonTasksIndexRoute;

#[lazy_route]
impl LazyRoute for BosonTasksIndexRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <BosonTasksIndexPage /> }.into_any()
    }
}

/// Lazy `/boson/tasks/:task_name`.
#[derive(Clone, Copy, Debug, Default)]
pub struct BosonTaskDetailRoute;

#[lazy_route]
impl LazyRoute for BosonTaskDetailRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <BosonTaskDetailPage /> }.into_any()
    }
}

/// Lazy `/boson/tasks/:task_name/config` (email-verified).
#[derive(Clone, Copy, Debug, Default)]
pub struct BosonVerifiedTaskConfigRoute;

#[lazy_route]
impl LazyRoute for BosonVerifiedTaskConfigRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! {
            <orbital::routes::RequireAuthenticated requires_email_verification=true>
                <BosonTaskConfigPage />
            </orbital::routes::RequireAuthenticated>
        }
        .into_any()
    }
}

/// Lazy `/boson/queue`.
#[derive(Clone, Copy, Debug, Default)]
pub struct BosonQueueRoute;

#[lazy_route]
impl LazyRoute for BosonQueueRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <BosonQueuePage /> }.into_any()
    }
}

/// Lazy `/boson/runs`.
#[derive(Clone, Copy, Debug, Default)]
pub struct BosonRunsIndexRoute;

#[lazy_route]
impl LazyRoute for BosonRunsIndexRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <BosonRunsIndexPage /> }.into_any()
    }
}

/// Lazy `/boson/runs/:id`.
#[derive(Clone, Copy, Debug, Default)]
pub struct BosonRunDetailRoute;

#[lazy_route]
impl LazyRoute for BosonRunDetailRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <BosonRunDetailPage /> }.into_any()
    }
}
