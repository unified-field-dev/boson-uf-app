//! Gluon virtual pool picker for Boson task configuration (app boundary).
//!
//! Gluon/Pion integration is deferred to Wave 7; this export returns the default
//! global pool option only so task-config UI remains usable.

use leptos::prelude::*;

#[cfg(feature = "ssr")]
use super::helpers::{require_email_verified, require_session, trace_server_result};
use super::types::GluonPoolPickRow;

/// Lists Gluon virtual pools suitable for Boson task routing.
#[uf_product_macros::server(permission = "BosonAdmin")]
pub async fn list_gluon_pools_for_boson_task_config() -> Result<Vec<GluonPoolPickRow>, ServerFnError>
{
    let result = async {
        let ctx = higgs::Higgs::from_request().await?;
        require_session(&ctx)?;
        require_email_verified().await?;
        Ok(boson_backend::default_gluon_pool_rows())
    }
    .await;
    #[cfg(feature = "ssr")]
    trace_server_result(
        "list_gluon_pools_for_boson_task_config",
        &result,
        None,
        None,
        None,
    );
    result
}
