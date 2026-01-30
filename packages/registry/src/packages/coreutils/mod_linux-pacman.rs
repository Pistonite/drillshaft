//! GNU Coreutils, Diffutils, and other basic commands

use crate::pre::*;

mod eza;
mod common;

register_binaries!(
    "ls", "diff", "find", "gzip", "sed", "grep", "zip", "unzip", "tar"
);

pub fn verify(_: &Context) -> cu::Result<Verified> {
    eza::verify()?;
    check_installed_pacman_package!("base");

    let v = check_installed_pacman_package!("zip");
    if Version(&v) < metadata::coreutils::zip::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    let v = check_installed_pacman_package!("unzip");
    if Version(&v) < metadata::coreutils::unzip::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    let v = check_installed_pacman_package!("tar");
    if Version(&v) < metadata::coreutils::tar::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    Ok(Verified::is_uptodate(common::ALIAS_VERSION.is_uptodate()?))
}

pub fn install(ctx: &Context) -> cu::Result<()> {
    eza::install(ctx)?;
    epkg::pacman::install("base", ctx.bar_ref())?;
    epkg::pacman::install("zip", ctx.bar_ref())?;
    epkg::pacman::install("unzip", ctx.bar_ref())?;
    epkg::pacman::install("tar", ctx.bar_ref())?;
    Ok(())
}

pub fn uninstall(_: &Context) -> cu::Result<()> {
    eza::uninstall()?;
    cu::warn!("not uninstalling the 'base' package for your sanity");
    Ok(())
}

pub fn configure(ctx: &Context) -> cu::Result<()> {
    eza::configure(ctx)?;

    let alias_version = hmgr::get_cached_version("coreutils-alias")?;
    if alias_version.as_deref() != Some(metadata::coreutils::ALIAS_VERSION) {
        // using shell alias for UI-only differences
        let grep_alias = "alias grep='grep --color=auto'";
        ctx.add_item(hmgr::Item::Bash(grep_alias.to_string()))?;
        ctx.add_item(hmgr::Item::Zsh(grep_alias.to_string()))?;
        hmgr::set_cached_version("coreutils-alias", metadata::coreutils::ALIAS_VERSION)?;
    }

    Ok(())
}
