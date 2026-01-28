//! Microsoft fork of Git

use crate::pre::*;

mod version;

static ALIAS_VERSION: VersionCache = VersionCache::new("git-alias", metadata::git::ALIAS_VERSION);

register_binaries!("git", "scalar", "bash");

pub fn verify(_: &Context) -> cu::Result<Verified> {
    check_bin_in_path!("git");
    // we don't check if the bash is in shaft, because WSL
    // litters a bash.exe into PATH (bruh)
    check_bin_in_path!("bash");
    let version = command_output!("git", ["--version"]);
    if !version.contains("vfs") {
        cu::bail!("current 'git' is not the vfs version (microsoft.git); please uninstall it or use the 'system-git' package");
    }
    check_bin_in_path!("scalar");
    let v = version::verify(metadata::git::VERSION)?;
    if v != Verified::UpToDate {
        return Ok(v);
    }

    Ok(Verified::is_uptodate(ALIAS_VERSION.is_uptodate()?))
}

pub fn install(ctx: &Context) -> cu::Result<()> {
    opfs::ensure_terminated("git.exe")?;
    opfs::ensure_terminated("scalar.exe")?;
    epkg::winget::install("Microsoft.Git", ctx.bar_ref())?;
    Ok(())
}

pub fn uninstall(ctx: &Context) -> cu::Result<()> {
    opfs::ensure_terminated("git.exe")?;
    opfs::ensure_terminated("scalar.exe")?;
    epkg::winget::uninstall("Microsoft.Git", ctx.bar_ref())?;
    Ok(())
}

pub fn configure(ctx: &Context) -> cu::Result<()> {
    if ctx.needs_configure(ALIAS_VERSION) {
        let exe_path = opfs::find_in_wingit("bin/bash.exe")?;
        ctx.add_item(hmgr::Item::ShimBin(
            bin_name!("bash").to_string(),
            vec![exe_path.into_utf8()?],
        ))?;
        ALIAS_VERSION.update()?;
    }
    Ok(())
}
