//! Permission manifest for the Boson operations app.

use uf_product_macros::UfPermissionManifest;

/// Admin permission for Boson mutating server functions.
///
/// Synced into the `boson` domain; gated with
/// `#[uf_product_macros::server(permission = "BosonAdmin")]`.
#[allow(clippy::expl_impl_clone_on_copy)]
#[derive(UfPermissionManifest)]
#[permission_manifest(
    domain_key = "boson",
    domain_name = "Boson",
    domain_description = "Boson background-work administration"
)]
pub enum BosonPermission {
    /// Cancel jobs and update task configuration.
    #[permission(description = "Administer Boson job cancellation and task configuration")]
    BosonAdmin,
}
