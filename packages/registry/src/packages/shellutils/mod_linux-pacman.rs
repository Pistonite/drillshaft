//! Additional essential shell utilities

use crate::pre::*;

#[rustfmt::skip]
register_binaries!(
    "perl", "gpg", "curl", "wget",
    "fzf", "jq",
    "bat", "dust", "fd", "websocat", "zoxide", "c", "ci",
    "viopen", "vibash", "n", "wsclip"
);

mod perl;

pub fn verify(_: &Context) -> cu::Result<Verified> {
    check_installed_pacman_package!("perl");
    let v = perl::version_check()?;
    if v != Verified::UpToDate {
        return Ok(v);
    }
    cu::check!(cu::which("gpg"), "gnupg is a dependency of Arch Linux and is not found")?;
    let v = check_installed_pacman_package!("curl");
    if Version(&v) < metadata::curl::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    let v = check_installed_pacman_package!("wget");
    if Version(&v) < metadata::wget::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    let v = check_installed_pacman_package!("fzf");
    if Version(&v) < metadata::fzf::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    let v = check_installed_pacman_package!("jq");
    if Version(&v) < metadata::jq::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    let v = check_installed_with_cargo!("bat");
    if Version(&v.version) < metadata::bat::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    let v = check_installed_with_cargo!("du-dust");
    if Version(&v.version) < metadata::dust::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    let v = check_installed_with_cargo!("fd-find");
    if Version(&v.version) < metadata::fd::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    let v = check_installed_with_cargo!("websocat");
    if Version(&v.version) < metadata::websocat::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    let v = check_installed_with_cargo!("zoxide");
    if Version(&v.version) < metadata::zoxide::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    let v = check_installed_with_cargo!("viopen");
    if Version(&v.version) < metadata::shellutils::viopen::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    // let alias_version = hmgr::get_cached_version("coreutils-alias")?;
    // Ok(Verified::is_uptodate(alias_version.as_deref() == Some(metadata::coreutils::ALIAS_VERSION)))
    todo!()
}

pub fn install(ctx: &Context) -> cu::Result<()> {
    epkg::pacman::install("perl", ctx.bar_ref())?;
    epkg::pacman::install("curl", ctx.bar_ref())?;
    epkg::pacman::install("wget", ctx.bar_ref())?;
    epkg::pacman::install("fzf", ctx.bar_ref())?;
    epkg::pacman::install("jq", ctx.bar_ref())?;
    epkg::cargo::binstall("bat", ctx.bar_ref())?;
    epkg::cargo::binstall("du-dust", ctx.bar_ref())?;
    epkg::cargo::install("fd-find", ctx.bar_ref())?;
    epkg::cargo::install("websocat", ctx.bar_ref())?;
    epkg::cargo::install("zoxide", ctx.bar_ref())?;
    epkg::cargo::install_git_commit("viopen", metadata::shellutils::REPO, metadata::shellutils::COMMIT, ctx.bar_ref())?;
    Ok(())
}

pub fn uninstall(ctx: &Context) -> cu::Result<()> {
    epkg::pacman::uninstall("perl", ctx.bar_ref())?;
    epkg::pacman::uninstall("curl", ctx.bar_ref())?;
    epkg::pacman::uninstall("wget", ctx.bar_ref())?;
    epkg::pacman::uninstall("fzf", ctx.bar_ref())?;
    epkg::pacman::uninstall("jq", ctx.bar_ref())?;
    epkg::cargo::uninstall("bat")?;
    epkg::cargo::uninstall("du-dust")?;
    epkg::cargo::uninstall("fd-find")?;
    epkg::cargo::uninstall("websocat")?;
    epkg::cargo::uninstall("zoxide")?;
    epkg::cargo::uninstall("viopen")?;
    Ok(())
}
