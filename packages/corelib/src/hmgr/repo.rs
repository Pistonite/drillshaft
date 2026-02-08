use cu::pre::*;

use crate::{bin_name, hmgr};

static SHAFT_REPO: &str = "https://github.com/Pistonite/shaft";

/// Files to remove before building
static FILES_TO_REMOVE: &[&str] = &[
    "packages/corelib/src/hmgr/tools_targz.gen.rs",
    "packages/corelib/src/hmgr/tools.tar.gz",
];

/// Build shaft from source locally and update the current executable
pub fn local_update() -> cu::Result<()> {
    let repo_path = hmgr::paths::repo();

    if !repo_path.exists() {
        cu::fs::make_dir(&repo_path)?;
        cu::which("git")?
            .command()
            .add(cu::args!["clone", SHAFT_REPO, &repo_path])
            .stdout(cu::lv::P)
            .stderr(cu::lv::P)
            .stdin_null()
            .wait_nz()?;
    }

    cu::which("git")?
        .command()
        .current_dir(&repo_path)
        .add(cu::args!["fetch", "origin", "main"])
        .stdout(cu::lv::P)
        .stderr(cu::lv::P)
        .stdin_null()
        .wait_nz()?;

    cu::which("git")?
        .command()
        .current_dir(&repo_path)
        .add(cu::args!["reset", "--hard", "origin/main"])
        .stdout(cu::lv::P)
        .stderr(cu::lv::P)
        .stdin_null()
        .wait_nz()?;

    for file in FILES_TO_REMOVE {
        let file_path = repo_path.join(file);
        if file_path.exists() {
            cu::debug!("removing: {}", file_path.display());
            cu::fs::remove(&file_path)?;
        }
    }

    let (child, bar) = cu::which("cargo")?
        .command()
        .current_dir(&repo_path)
        .add(cu::args![
            "build",
            "--bin",
            "shaft",
            "--release",
            "--locked"
        ])
        .preset(cu::pio::cargo("ensure successful build"))
        .spawn()?;
    child.wait_nz()?;
    bar.done();

    let mut built_binary = repo_path.clone();
    built_binary.extend(["target", "release", bin_name!("shaft")]);
    if !built_binary.exists() {
        cu::bail!("built binary not found at: {}", built_binary.display());
    }

    let current_exe = std::env::current_exe()?;
    let exe_old = current_exe.with_extension(if cfg!(windows) { "old.exe" } else { "old" });

    if exe_old.exists() {
        cu::check!(
            cu::fs::remove(&exe_old),
            "failed to remove old executable at '{}'",
            exe_old.display()
        )?;
    }

    std::fs::rename(&current_exe, &exe_old)?;
    cu::fs::copy(&built_binary, &current_exe)?;

    let package_path = repo_path.join("packages/cli");
    cu::which("cargo")?
        .command()
        .add(cu::args!["install", "--path", &package_path, "--locked"])
        .stdout(cu::lv::P)
        .stderr(cu::lv::P)
        .stdin_null()
        .wait_nz()?;

    cu::info!("update successful");
    Ok(())
}
