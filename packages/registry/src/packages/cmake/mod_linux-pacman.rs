//! CMake makefile generator
use crate::pre::*;
register_binaries!("cmake");

pub fn verify(_: &Context) -> cu::Result<Verified> {
    let v = check_installed_pacman_package!("cmake");
    check_outdated!(&v, metadata::cmake::VERSION);
    Ok(Verified::UpToDate)
}
pub fn install(ctx: &Context) -> cu::Result<()> {
    epkg::pacman::install("cmake", ctx.bar_ref())?;
    Ok(())
}
pub fn uninstall(ctx: &Context) -> cu::Result<()> {
    cu::warn!("not uninstalling cmake for your sanity");
    Ok(())
}
