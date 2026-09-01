//! Process-local overrides for `boson-uf-app-e2e` Playwright seeds.
//!
//! Enabled only with the `e2e-lab` Cargo feature (lab host). Production hosts
//! leave the feature off so setters are absent and overrides never apply.

#[cfg(feature = "e2e-lab")]
use std::sync::atomic::{AtomicI8, Ordering};

/// `-1` = unset (use lepton-auth); `0` = force unverified; `1` = force verified.
#[cfg(feature = "e2e-lab")]
static EMAIL_VERIFIED: AtomicI8 = AtomicI8::new(-1);

/// Set by `POST /api/test/seed-data` in boson-uf-app-e2e only.
#[cfg(feature = "e2e-lab")]
pub fn set_email_verified_override(verified: Option<bool>) {
    let v = match verified {
        None => -1,
        Some(false) => 0,
        Some(true) => 1,
    };
    EMAIL_VERIFIED.store(v, Ordering::SeqCst);
}

#[cfg(feature = "e2e-lab")]
pub(crate) fn email_verified_override() -> Option<bool> {
    match EMAIL_VERIFIED.load(Ordering::SeqCst) {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

/// Always unset when the lab feature is disabled.
#[cfg(not(feature = "e2e-lab"))]
#[must_use]
pub(crate) const fn email_verified_override() -> Option<bool> {
    None
}
