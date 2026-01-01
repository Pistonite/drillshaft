use cu::pre::*;
use registry::{Context, PkgId, Verified};

pub fn sync(packages: &[String]) -> cu::Result<()> {
    let pkgs = crate::graph::parse_pkgs(packages)?;
    let mut installed = crate::graph::InstallCache::load()?;
    let graph = crate::graph::build_sync_graph(pkgs, &installed)?;
    match graph.len() {
        1 => cu::info!("syncing 1 package..."),
        x => cu::info!("syncing {x} packages..."),
    }
    for pkg in graph {
        cu::check!(do_sync_package(pkg), "failed to sync '{pkg}'")?;
        installed.add_to_installed(pkg);
        installed.save()?;
    }
    Ok(())
}

fn do_sync_package(pkg: PkgId) -> cu::Result<()> {
    let package = pkg.package();
    let ctx = Context { pkg };
    match package.verify(&ctx)? {
        Verified::UpToDate => {
            // TODO: check config dirty
            cu::info!("up-to-date: '{pkg}'");
            return Ok(());
        }
        Verified::NotUpToDate => {
            // TODO: backup
            cu::debug!("needs-update: '{pkg}'");
        }
        Verified::NotInstalled => {}
    }
    let _progress = cu::progress_unbounded(format!("sync '{pkg}'"));

    package.download(&ctx)?;
    package.build(&ctx)?;
    package.install(&ctx)?;
    package.configure(&ctx)?;
    package.clean(&ctx)?;

    match package.verify(&ctx)? {
        Verified::UpToDate => {}
        _ => {
            cu::bail!("verification failed after installation");
        }
    }

    Ok(())
}
