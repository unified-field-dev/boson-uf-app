//! Protected `/boson` host: session auth gate + in-memory dashboard happy path.
//!
//! Copy surfaces for product hosts: this package's `Cargo.toml` + `main.rs`,
//! plus the product-mount dependency / Leptos sketches in the host README.
//! Oneshot path `/boson` matches Orbital app id/path `boson` / `/boson`
//! (see JSON `inventory`).
//!
//! Mirrors what a real host does before mounting [`boson_app::BosonRoutes`]:
//! deny anonymous traffic under `/boson`, then serve the dashboard KPI shape
//! the UI's `get_dashboard_stats` server fn builds via `boson-backend`.
//!
//! ## When to use
//! Smoke the `/boson` auth + dashboard contract without a full Leptos SSR graph.
//!
//! ## Command
//! ```bash
//! export CARGO_BUILD_JOBS=1
//! export CARGO_TARGET_DIR=target-boson-uf-app
//! cargo run -p protected-boson-host
//! ```
//!
//! ## Success
//! Stdout prints `protected_boson_host: OK — /boson deny/allow + dashboard KPIs`.
//!
//! ## Look next
//! Mount `<BosonRoutes />` in a product host; wire Higgs + Boson coordinator for live data.

#![allow(clippy::print_stdout, clippy::unwrap_used, clippy::expect_used)]

use axum::body::Body;
use axum::extract::Extension;
use axum::http::{Request, StatusCode};
use axum::middleware::{from_fn, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Json;
use axum::Router;
use http_body_util::BodyExt;
use tower::ServiceExt;

#[derive(Clone)]
struct DemoSession {
    user_id: String,
}

async fn require_session(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    if req.extensions().get::<DemoSession>().is_some() {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn inject_demo_session(mut req: Request<Body>, next: Next) -> Response {
    if let Some(user) = req
        .headers()
        .get("x-demo-user")
        .and_then(|v| v.to_str().ok())
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
    {
        req.extensions_mut().insert(DemoSession { user_id: user });
    }
    next.run(req).await
}

async fn boson_dashboard(Extension(session): Extension<DemoSession>) -> impl IntoResponse {
    // Same KPI builder the Leptos `get_dashboard_stats` server fn calls after Higgs auth.
    let stats = boson_backend::dashboard_stats(3, 2, 1, 5);
    Json(serde_json::json!({
        "path": "/boson",
        "user": session.user_id,
        "stats": stats,
        "inventory": {
            "app_id": "boson",
            "route_path": "/boson",
            "auth_gate": "RequireAuthenticated",
            "admin_permission": "BosonAdmin",
        },
    }))
}

fn app() -> Router {
    Router::new()
        .route("/boson", get(boson_dashboard))
        .route_layer(from_fn(require_session))
        .layer(from_fn(inject_demo_session))
}

async fn status_for(path: &str, user: Option<&str>) -> StatusCode {
    let mut builder = Request::builder().uri(path);
    if let Some(user) = user {
        builder = builder.header("x-demo-user", user);
    }
    app()
        .oneshot(builder.body(Body::empty()).expect("req"))
        .await
        .expect("oneshot")
        .status()
}

#[tokio::main]
async fn main() {
    let denied = status_for("/boson", None).await;
    assert_eq!(denied, StatusCode::UNAUTHORIZED);

    let response = app()
        .oneshot(
            Request::builder()
                .uri("/boson")
                .header("x-demo-user", "demo-ops")
                .body(Body::empty())
                .expect("req"),
        )
        .await
        .expect("oneshot");
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(body["path"], "/boson");
    assert_eq!(body["user"], "demo-ops");
    assert_eq!(body["stats"]["task_count"], 3);
    assert_eq!(body["stats"]["jobs_queued"], 2);
    assert_eq!(body["stats"]["jobs_running"], 1);
    assert_eq!(body["stats"]["runs_today"], 5);
    assert_eq!(body["inventory"]["app_id"], "boson");
    assert_eq!(body["inventory"]["route_path"], "/boson");
    assert_eq!(body["inventory"]["auth_gate"], "RequireAuthenticated");
    assert_eq!(body["inventory"]["admin_permission"], "BosonAdmin");

    println!("protected_boson_host: OK — /boson deny/allow + dashboard KPIs");
}
