//! Boson ops UI Playwright host.
#![allow(missing_docs)]

mod app;
mod boson_routes_eager;
#[cfg(feature = "ssr")]
mod e2e_valence;
mod gate_demos;
mod harness_auth_menu;
#[cfg(feature = "ssr")]
pub mod seed;

pub use app::{shell, wire_gauge_permissions_bridge, App};
#[cfg(feature = "ssr")]
pub use e2e_valence::{e2e_boson_backend, e2e_higgs_config, e2e_router, init_e2e_valence};
#[cfg(feature = "ssr")]
pub use gate_demos::inject_e2e_session_snapshot;
