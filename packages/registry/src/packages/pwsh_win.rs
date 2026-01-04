//! PowerShell 7

use crate::pre::*;

// using preview version to enable tilde (~) expansion
static VERSION: &str = "7.6.0-preview.6";

register_binaries!("pwsh");

pub fn verify(_: &Context) -> cu::Result<Verified> {
    check_bin_in_path_and_shaft!("pwsh");
    let version = command_output!("pwsh", ["-NoLogo", "-NoProfile", "-c", "$PSVersionTable.PSVersion.ToString()"]);
    let is_preview = version.contains("preview");
    let is_uptodate = Version(version.trim()) >= VERSION;
    Ok(Verified::uptodate(is_preview && is_uptodate))
}
pub fn download(ctx: &Context) -> cu::Result<()> {
    let sha256_checksum = if cfg!(target_arch = "aarch64") {
        "36dc90e7f0e7870b0970c9a58790de4de4217e65acafaf790e87b7c97d93649f"
    } else {
        "481ce45bd9ebfab9a5b254a35f145fb6259bd452ae67d92ab1d231b6367987d9"
    };
    let arch = if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        "x64"
    };
    let url = format!("https://github.com/PowerShell/PowerShell/releases/download/v{VERSION}/PowerShell-{VERSION}-win-{arch}.zip");
    todo!()
}

pub fn install(ctx: &Context) -> cu::Result<()> {
    opfs::ensure_terminated("pwsh.exe")?;
    cu::warn!("please also install/update HackNerdFont:\n  https://github.com/ryanoasis/nerd-fonts/releases");
    let _ = cu::prompt!("press ENTER when confirmed HackNerdFont is installed")?;
    cu::check!(verify(ctx), "system-git requires 'git' to be installed on the system")?;
    Ok(())
}

pub fn uninstall(_: &Context) -> cu::Result<()> {
    Ok(())
}
