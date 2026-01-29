//! Pseudo package for the package manager itself

mod common;
pub use common::{config_location, install, pre_uninstall, pre_uninstall as uninstall, verify};

crate::register_binaries!("sudo", "cargo");
