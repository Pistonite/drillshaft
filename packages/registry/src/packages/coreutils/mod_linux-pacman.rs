//! GNU Coreutils, Diffutils, and other basic commands

use crate::pre::*;

mod eza;

register_binaries!("ls", "diff", "gzip", "sed", "grep");

pub fn verify(_: &Context) -> cu::Result<Verified> {
    eza::verify()?;
    check_installed_pacman_package!("base");
    let alias_version = hmgr::get_cached_version("coreutils-alias")?;
    Ok(Verified::is_uptodate(alias_version.as_deref() == Some(metadata::coreutils::ALIAS_VERSION)))
}

pub fn install(ctx: &Context) -> cu::Result<()> {
    eza::install(ctx)?;
    epkg::pacman::install("base", ctx.bar_ref())?;
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
