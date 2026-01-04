//! Configuration for Terminal

use crate::pre::*;

pub fn verify(_: &Context) -> cu::Result<Verified> {
    let installed = epkg::pacman::is_installed("ttf-hack-nerd")?;
    Ok(Verified::installed(installed))
}
pub fn install(_: &Context) -> cu::Result<()> {
    epkg::pacman::install("ttf-hack-nerd")
}
pub fn uninstall(_: &Context) -> cu::Result<()> {
    epkg::pacman::uninstall("ttf-hack-nerd")
}
