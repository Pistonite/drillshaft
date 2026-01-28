use std::sync::Arc;

use cu::pre::*;

/// Install a winget package
#[cu::context("failed to install {id} with winget")]
pub fn install(id: &str, bar: Option<&Arc<cu::ProgressBar>>) -> cu::Result<()> {
    let (child, bar, _) = cu::which("winget")?
        .command()
        // we can only --force because winget doesn't have a --needed flag (bruh)
        .args(["install", "--id", id, "--force"])
        .stdoe(
            cu::pio::spinner(format!("winget install '{id}'"))
                .configure_spinner(|builder| builder.parent(bar.cloned())),
        )
        .stdin_null()
        .spawn()?;
    if let Err(e) = child.wait_nz() {
        cu::error!("winget failed!");
        cu::hint!(
            "reboot and try again, or run `winget install --id {id} --force` manually and inspect the result."
        );
        cu::rethrow!(e);
    }
    bar.done();
    cu::info!("installed {id} with winget");
    Ok(())
}

/// Uninstall a winget package
#[cu::context("failed to uninstall {id} with winget")]
pub fn uninstall(id: &str, bar: Option<&Arc<cu::ProgressBar>>) -> cu::Result<()> {
    let (child, bar, _) = cu::which("winget")?
        .command()
        .args(["uninstall", id])
        .stdoe(
            cu::pio::spinner(format!("winget install '{id}'"))
                .configure_spinner(|builder| builder.parent(bar.cloned())),
        )
        .stdin_null()
        .spawn()?;
    if let Err(e) = child.wait_nz() {
        cu::error!("winget failed!");
        cu::hint!(
            "reboot and try again, or run `winget uninstall {id}` manually and inspect the result."
        );
        cu::rethrow!(e);
    }
    bar.done();
    cu::info!("uninstalled {id} with winget");
    Ok(())
}
