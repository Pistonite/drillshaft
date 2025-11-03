use cu::pre::*;

use crate::pre::*;

static GIT_WINDOWS_VERSION: &str = "2.51.2.windows";
static GIT_MICROSOFT_VERSION: &str = "2.51.2.vfs";
static GIT_VERSION: &str = "2.51.2";

pub fn verify(ctx: &Context) -> cu::Result<Verified> {
    let Ok(git) = cu::which("git") else {
        return Ok(Verified::NotInstalled);
    };
    let (child, stdout) = git
        .command()
        .arg("--version")
        .stdout(cu::pio::string())
        .stdie_null()
        .spawn()?;
    child.wait_nz()?;
    let stdout = stdout.join()??;
    let version = stdout.strip_prefix("git version ").unwrap_or(&stdout);

    if ctx.platform != Platform::Windows {
        if version.is_version_same_or_higher_than(GIT_VERSION) {
            return Ok(Verified::UpToDate);
        }
        return Ok(Verified::NotUpToDate);
    }
    let min_version = if version.contains("vfs") {
        GIT_MICROSOFT_VERSION
    } else {
        GIT_WINDOWS_VERSION
    };
    if version.is_version_same_or_higher_than(min_version) {
        Ok(Verified::UpToDate)
    }
    else {
        Ok(Verified::NotUpToDate)
    }
}
