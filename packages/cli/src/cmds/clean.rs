use corelib::ItemMgr;
use registry::{Context, PkgId, Stage};

use crate::graph::{self, InstallCache};

pub fn clean(packages: &[String], all: bool) -> cu::Result<()> {
    let installed = InstallCache::load()?;
    let mut pkgs = if all {
        let mut p = installed.pkgs;
        p.insert(PkgId::Core); // clean core with --all
        p
    } else {
        graph::parse_pkgs(packages)?
    };
    let clean_core = if pkgs.is_empty() {
        true
    } else {
        pkgs.remove(PkgId::Core)
    };
    if !pkgs.is_empty() {
        let items = ItemMgr::load()?;
        let mut ctx = Context::new(items);
        for pkg in installed.pkgs {
            ctx.set_installed(pkg, true);
        }
        for pkg in pkgs {
            if !installed.pkgs.contains(pkg) {
                cu::warn!("not installed: '{pkg}'");
                continue;
            }
            ctx.pkg = pkg;
            let bar = cu::progress(format!("clean '{pkg}'")).spawn();
            ctx.set_bar(Some(&bar));
            ctx.stage.set(Stage::Clean);
            if let Err(e) = pkg.package().clean(&ctx) {
                cu::warn!("failed to clean package {pkg}: {e:?}");
            }
            bar.done();
        }
    }

    if clean_core {
        corelib::hmgr::clean_home();
    }

    Ok(())
}
