use std::path::PathBuf;

use corelib::hmgr;
use cu::pre::*;

use crate::PkgId;

/// Context passed to package functions
pub struct Context {
    /// The id of the package being operated on
    pub pkg: PkgId,
}
impl Context {
    pub fn pkg_name(&self) -> &'static str {
        self.pkg.to_str()
    }
    pub fn temp_dir(&self) -> PathBuf {
        hmgr::paths::temp_dir(self.pkg_name())
    }
    pub fn install_dir(&self) -> PathBuf {
        hmgr::paths::install_dir(self.pkg_name())
    }
    pub fn install_old_dir(&self) -> PathBuf {
        hmgr::paths::install_old_dir(self.pkg_name())
    }
    /// Move HOME/install/<package> directory to HOME/install-old/<package>,
    /// if it exists. The old old will be deleted
    pub fn move_install_to_old_if_exists(&self) -> cu::Result<()> {
        let cur_install_dir = self.install_dir();
        if !cur_install_dir.exists() {
            return Ok(());
        }
        cu::debug!("moving install dir to old: '{}'", cur_install_dir.display());
        let old_install_dir = self.install_old_dir();
        let old_install_root = hmgr::paths::install_old_root();
        cu::check!(
            cu::fs::make_dir(old_install_root),
            "failed to create old install root"
        )?;
        cu::check!(
            cu::fs::rec_remove(&old_install_dir),
            "failed to remove old install dir"
        )?;
        cu::check!(
            cu::fs::rename(cur_install_dir, old_install_dir),
            "failed to move install dir to install-old"
        )?;
        Ok(())
    }
}
