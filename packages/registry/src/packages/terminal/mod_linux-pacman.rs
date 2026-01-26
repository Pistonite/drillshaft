//! Configuration for Terminal

use crate::pre::*;

pub fn verify(_: &Context) -> cu::Result<Verified> {
    let version = check_installed_pacman_package!("ttf-hack-nerd");
    Ok(Verified::is_uptodate(!(Version(version.trim()) < metadata::hack_font::VERSION_PACMAN)))
}
pub fn install(ctx: &Context) -> cu::Result<()> {
    epkg::pacman::install("ttf-hack-nerd", ctx.bar_ref())
}
pub fn uninstall(ctx: &Context) -> cu::Result<()> {
    epkg::pacman::uninstall("ttf-hack-nerd", ctx.bar_ref())
}
