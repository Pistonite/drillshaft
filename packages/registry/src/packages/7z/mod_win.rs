//! 7-Zip
use crate::pre::*;

mod version;

register_binaries!("7z", "7zfm");

static VERSION: &str = "25.01";

pub fn verify(_: &Context) -> cu::Result<Verified> {
    check_bin_in_path_and_shaft!("7z");
    check_bin_in_path_and_shaft!("7zfm");
    version::check(VERSION)
}
pub fn install(_: &Context) -> cu::Result<()> {
    opfs::ensure_terminated("7z")?;
    epkg::pacman::install("7zip")?;
    Ok(())
}
pub fn uninstall(_: &Context) -> cu::Result<()> {
    opfs::ensure_terminated("7z")?;
    epkg::pacman::uninstall("7zip")?;
    Ok(())
}
