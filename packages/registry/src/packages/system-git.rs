//! Use `git` found in PATH
use crate::pre::*;

register_binaries!("git");

pub fn verify(_: &Context) -> cu::Result<Verified> {
    check_in_path!("git");
    Ok(Verified::UpToDate)
}

pub fn install(ctx: &Context) -> cu::Result<()> {
    cu::check!(verify(ctx), "system-git requires `git` in PATH")?;
    Ok(())
}

pub fn uninstall(_: &Context) -> cu::Result<()> {
    Ok(())
}
