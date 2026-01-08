use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

use cu::pre::*;

use crate::{bin_name, hmgr, opfs};

pub struct ShimConfig {
    config: BTreeMap<String, Vec<String>>,
    is_dirty: bool,
}
impl ShimConfig {
    pub fn load() -> cu::Result<Self> {
        let config_path = hmgr::paths::shim_config_json();
        let config = match cu::fs::read_string(&config_path) {
            Ok(s) => json::parse::<BTreeMap<String, Vec<String>>>(&s)?,
            Err(_) => Default::default(),
        };

        Ok(Self {
            config,
            is_dirty: false,
        })
    }
    /// Add new shim entry. Marks self as dirty but does NOT rebuild the shim executable
    pub fn add(&mut self, name: &str, args: &[&str]) {
        self.config.insert(
            bin_name!(name),
            args.iter().map(|x| x.to_string()).collect(),
        );
        self.is_dirty = true;
    }
    /// Remove shim entry. Marks self as dirty but does NOT rebuild the shim executable.
    /// However, the old symlink will be deleted if exists
    pub fn remove(&mut self, name: &str) -> cu::Result<()> {
        let bin_name = bin_name!(name);
        self.config.remove(&bin_name);
        self.is_dirty = true;
        cu::fs::remove(hmgr::paths::binary(bin_name))?;
        Ok(())
    }
    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }
    pub fn build(&mut self) -> cu::Result<()> {
        if !self.is_dirty() {
            return Ok(());
        }
        let config_path = hmgr::paths::shim_config_json();
        cu::fs::write_json_pretty(&config_path, &self.config)?;

        hmgr::tools::ensure_unpacked()?;
        let mut shim_path = hmgr::paths::tools_root();
        shim_path.push("shim-build");

        let (child, _, _) = cu::which("cargo")?
            .command()
            .env("SHAFT_SHIM_BUILD_CONFIG", &config_path)
            .add(cu::args![
                "build",
                "--release",
                "--manifest-path",
                shim_path.join("Cargo.toml")
            ])
            .name("building shim executable")
            .preset(cu::pio::cargo())
            .spawn()?;
        child.wait_nz()?;

        shim_path.extend(["target", "release"]);
        shim_path.push(bin_name!("shaftim"));
        cu::fs::copy(&shim_path, hmgr::paths::shim_binary())?;
        self.sync_links()?;
        self.is_dirty = false;
        Ok(())
    }

    /// Clean the bin directory of old symlinks, and make new links if needed
    pub fn sync_links(&self) -> cu::Result<()> {
        let target = hmgr::paths::shim_binary();
        let mut need_to_link: BTreeSet<_> = self.config.keys().collect();
        let mut need_to_remove = BTreeSet::new();
        let bin_root = hmgr::paths::bin_root();
        let bin_dir = cu::fs::read_dir(&bin_root)?;
        for entry in bin_dir {
            let entry = entry?;
            if !entry.file_type()?.is_symlink() {
                continue;
            }
            let Ok(link_target) = std::fs::read_link(entry.path()) else {
                continue;
            };
            let link_target = link_target.normalize()?;
            if link_target != target {
                continue;
            }
            let Ok(file_name) = entry.file_name().into_string() else {
                continue;
            };
            if !self.config.contains_key(&file_name) {
                need_to_remove.insert(file_name);
                continue;
            }
            need_to_link.remove(&file_name);
        }
        for file_name in need_to_remove {
            let path = bin_root.join(&file_name);
            cu::fs::remove(path)?;
            cu::info!("removed old binary symlink '{file_name}'");
        }
        if !need_to_link.is_empty() {
            let paths: Vec<_> = need_to_link.iter().map(|x| bin_root.join(x)).collect();
            let links: Vec<_> = paths
                .iter()
                .map(|x| (x.as_path(), target.as_path()))
                .collect();
            opfs::symlink_files(&links)?;
            cu::info!("created new binary symlinks");
        }
        Ok(())
    }
}
