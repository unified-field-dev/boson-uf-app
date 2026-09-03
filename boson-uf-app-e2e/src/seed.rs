//! Harness-only seed endpoint for Playwright.

use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use crate::e2e_valence::{e2e_fixtures, refresh_queue_job, store_fixtures};
use crate::gate_demos::{write_e2e_auth_kind, E2eAuthKind};

#[derive(Debug, Deserialize)]
pub struct SeedRequest {
    /// `anonymous` | `admin` | `outsider` | `unverified`
    #[serde(default = "default_auth")]
    pub auth: String,
    /// When true, enqueue a fresh queued job for cancel specs.
    #[serde(default)]
    pub refresh_job: bool,
}

fn default_auth() -> String {
    E2eAuthKind::Anonymous.as_str().to_string()
}

pub async fn seed_data(
    session: tower_sessions::Session,
    Json(body): Json<SeedRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let kind = E2eAuthKind::parse(&body.auth);
    write_e2e_auth_kind(&session, kind)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Lab seam: skip lepton-auth Backend for task-config server fns.
    match kind {
        E2eAuthKind::Anonymous => boson_app::e2e_lab::set_email_verified_override(None),
        E2eAuthKind::Admin | E2eAuthKind::Outsider => {
            boson_app::e2e_lab::set_email_verified_override(Some(true));
        }
        E2eAuthKind::Unverified => {
            boson_app::e2e_lab::set_email_verified_override(Some(false));
        }
    }

    let mut fixtures = e2e_fixtures();
    if body.refresh_job {
        fixtures = refresh_queue_job().await.map_err(|e| {
            log::error!("seed refresh_job failed: {e:#}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        store_fixtures(fixtures.clone());
    }

    Ok(Json(serde_json::json!({
        "ok": true,
        "auth": kind.as_str(),
        "fixtures": {
            "task_name": fixtures.task_name,
            "job_id": fixtures.job_id,
            "run_id": fixtures.run_id,
        }
    })))
}
