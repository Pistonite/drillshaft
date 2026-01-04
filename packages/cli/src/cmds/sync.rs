use cu::pre::*;
use enumset::EnumSet;
use registry::{Context, PkgId, Verified};

use crate::graph::{self, InstallCache};

pub fn sync(packages: &[String]) -> cu::Result<()> {
    let pkgs = graph::parse_pkgs(packages)?;
    let mut installed = InstallCache::load()?;
    let pkgs = if pkgs.is_empty() {
        // sync all installed packages
        installed.pkgs
    } else {
        pkgs
    };
    sync_pkgs(pkgs, &mut installed)
}

pub fn sync_pkgs(pkgs: EnumSet<PkgId>, installed: &mut InstallCache) -> cu::Result<()> {
    if pkgs.is_empty() {
        return Ok(());
    }
    let graph = graph::build_sync_graph(pkgs, &installed, &mut Default::default())?;
    match graph.len() {
        1 => cu::info!("syncing 1 package..."),
        x => cu::info!("syncing {x} packages..."),
    }
    for pkg in graph {
        cu::check!(do_sync_package(pkg), "failed to sync '{pkg}'")?;
        installed.add(pkg)?;
        installed.save()?;
    }
    Ok(())
}

fn do_sync_package(pkg: PkgId) -> cu::Result<()> {
    let package = pkg.package();
    let ctx = Context { pkg };
    let needs_backup = match package.verify(&ctx)? {
        Verified::UpToDate => {
            // TODO: check config dirty
            cu::info!("up to date: '{pkg}'");
            return Ok(());
        }
        Verified::NotUpToDate => {
            cu::debug!("needs update: '{pkg}'");
            true
        }
        Verified::NotInstalled => { false }
    };
    let total = if needs_backup { 6 }else {5};
    let bar = cu::progress_bar(total, format!("sync '{pkg}'"));
    let mut i = 0;
    let backup_guard = if needs_backup {
        cu::progress!(&bar, i, "backup");
        i += 1;
        Some(package.backup_guard(&ctx)?)
    } else { None };

    cu::progress!(&bar, i, "downloading");
    i += 1;
    package.download(&ctx)?;
    cu::progress!(&bar, i, "building");
    i += 1;
    package.build(&ctx)?;
    cu::progress!(&bar, i, "installing");
    i += 1;
    package.install(&ctx)?;
    cu::progress!(&bar, i, "configuring");
    i += 1;
    package.configure(&ctx)?;
    cu::progress!(&bar, i, "cleaning");
    package.clean(&ctx)?;

    match package.verify(&ctx)? {
        Verified::UpToDate => {
            cu::progress_done!(&bar, "synced '{pkg}'");
            if let Some(mut x) = backup_guard {
                x.clear();
            }
        }
        _ => {
            cu::bail!("verification failed after installation");
        }
    }

    Ok(())
}
