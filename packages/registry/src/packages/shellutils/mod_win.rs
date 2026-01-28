//! Additional essential shell utilities

use crate::pre::*;

static ALIAS_VERSION: VersionCache = VersionCache::new("shellutils-alias", metadata::shellutils::ALIAS_VERSION);

#[rustfmt::skip]
register_binaries!(
    "perl", "gpg", "curl", "wget",
    "fzf", "jq",
    "bat", "dust", "fd", "websocat", "zoxide", "c", "ci",
    "viopen", "vipwsh", "vihosts", "n",
    "vipath",
    "wsclip"
);

mod perl;
mod wget;

pub fn binary_dependencies() -> EnumSet<BinId> {
    enum_set! { BinId::Scalar | BinId::_7z }
}

pub fn verify(_: &Context) -> cu::Result<Verified> {
    check_bin_in_path_and_shaft!("perl");
    let v = perl::version_check()?;
    if v != Verified::UpToDate {
        return Ok(v);
    }
    check_bin_in_path_and_shaft!("gpg");
    cu::check!(cu::which("curl"), "curl.exe is bundled in Windows; your Windows version might be too low")?;

    check_bin_in_path_and_shaft!("wget");
    let v = wget::version_check()?;
    if v != Verified::UpToDate {
        return Ok(v);
    }

    check_bin_in_path_and_shaft!("fzf");
    let v = command_output!("fzf", ["--version"]);
    let v = v.split_once(' ').map(|x| x.0).unwrap_or(&v);
    if Version(&v) < metadata::fzf::VERSION {
        return Ok(Verified::NotUpToDate);
    }

    check_bin_in_path_and_shaft!("jq");
    let v = command_output!("jq", ["--version"]);
    let v = v.strip_prefix("jq-").unwrap_or(&v);
    if Version(&v) < metadata::jq::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    let v = check_installed_with_cargo!("bat");
    if Version(&v.version) < metadata::bat::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    let v = check_installed_with_cargo!("dust", "du-dust");
    if Version(&v.version) < metadata::dust::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    let v = check_installed_with_cargo!("find", "fd-find");
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
    let v = check_installed_with_cargo!("n");
    if Version(&v.version) < metadata::shellutils::n::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    let v = check_installed_with_cargo!("wsclip");
    if Version(&v.version) < metadata::shellutils::wsclip::VERSION {
        return Ok(Verified::NotUpToDate);
    }
    let v = check_installed_with_cargo!("vipath");
    if Version(&v.version) < metadata::shellutils::vipath::VERSION {
        return Ok(Verified::NotUpToDate);
    }

    Ok(Verified::is_uptodate(ALIAS_VERSION.is_uptodate()?))
}
pub fn download(ctx: &Context) -> cu::Result<()> {
    hmgr::download_file("wget.7z", metadata::wget::URL, metadata::wget::SHA, ctx.bar())?;
    hmgr::download_file("fzf.zip", fzf_url(), metadata::fzf::SHA, ctx.bar())?;
    hmgr::download_file("jq.exe", jq_url(), metadata::jq::SHA, ctx.bar())?;
    Ok(())
}

pub fn install(ctx: &Context) -> cu::Result<()> {
    let install_dir = ctx.install_dir();
    cu::fs::make_dir(&install_dir)?;
    let wget_7z = hmgr::paths::download("wget.7z", metadata::wget::URL);
    opfs::un7z(wget_7z, &install_dir, ctx.bar_ref())?;
    let fzf_zip = hmgr::paths::download("fzf.zip", fzf_url());
    opfs::un7z(fzf_zip, &install_dir, ctx.bar_ref())?;
    let jq_exe = hmgr::paths::download("jq.exe", jq_url());
    let jq_target = install_dir.join(bin_name!("jq"));
    cu::fs::copy(jq_exe, jq_target)?;
    epkg::cargo::binstall("bat", ctx.bar_ref())?;
    epkg::cargo::binstall("du-dust", ctx.bar_ref())?;
    epkg::cargo::install("fd-find", ctx.bar_ref())?;
    epkg::cargo::install("websocat", ctx.bar_ref())?;
    epkg::cargo::install("zoxide", ctx.bar_ref())?;
    epkg::cargo::install_git_commit("viopen", metadata::shellutils::REPO, metadata::shellutils::COMMIT, ctx.bar_ref())?;
    epkg::cargo::install_git_commit("n", metadata::shellutils::REPO, metadata::shellutils::COMMIT, ctx.bar_ref())?;
    epkg::cargo::install_git_commit("wsclip", metadata::shellutils::REPO, metadata::shellutils::COMMIT, ctx.bar_ref())?;
    epkg::cargo::install_git_commit("vipath", metadata::shellutils::REPO, metadata::shellutils::COMMIT, ctx.bar_ref())?;
    Ok(())
}

pub fn uninstall(_: &Context) -> cu::Result<()> {
    epkg::cargo::uninstall("bat")?;
    epkg::cargo::uninstall("du-dust")?;
    epkg::cargo::uninstall("fd-find")?;
    epkg::cargo::uninstall("websocat")?;
    epkg::cargo::uninstall("zoxide")?;
    epkg::cargo::uninstall("viopen")?;
    epkg::cargo::uninstall("n")?;
    epkg::cargo::uninstall("wsclip")?;
    epkg::cargo::uninstall("vipath")?;
    Ok(())
}

pub fn configure(ctx: &Context) -> cu::Result<()> {
    if ctx.needs_configure(ALIAS_VERSION) {
        let exe_path = opfs::find_in_wingit("usr/bin/perl.exe")?;
        ctx.add_item(hmgr::Item::ShimBin(
            bin_name!("perl").to_string(),
            vec![exe_path.into_utf8()?],
        ))?;
        let exe_path = opfs::find_in_wingit("usr/bin/gpg.exe")?;
        ctx.add_item(hmgr::Item::ShimBin(
            bin_name!("gpg").to_string(),
            vec!["/bash/".to_string(), exe_path.into_utf8()?],
        ))?;
        ctx.add_item(hmgr::Item::LinkBin(
            hmgr::paths::binary(bin_name!("wget")).into_utf8()?,
            ctx.install_dir().join(bin_name!("wget")).into_utf8()?
        ))?;
        ctx.add_item(hmgr::Item::LinkBin(
            hmgr::paths::binary(bin_name!("fzf")).into_utf8()?,
            ctx.install_dir().join(bin_name!("fzf")).into_utf8()?
        ))?;
        ctx.add_item(hmgr::Item::LinkBin(
            hmgr::paths::binary(bin_name!("jq")).into_utf8()?,
            ctx.install_dir().join(bin_name!("jq")).into_utf8()?
        ))?;

        ctx.add_item(hmgr::Item::UserEnvVar("EDITOR".to_string(), "viopen".to_string()))?;

        // zoxide needs to be after starship, recommended to be at the end
        let script = command_output!("zoxide", ["init", "powershell", "--cmd", "c"]);
        ctx.add_priority_item(-1, hmgr::Item::Pwsh(script))?;

        ctx.add_item(hmgr::Item::ShimBin(bin_name!("vihosts").to_string(), vec![
            cu::which("cmd")?.into_utf8()?,
            "/c".to_string(),
            "viopen %SystemDrive%\\Windows\\System32\\drivers\\etc\\hosts".to_string()
        ]))?;

        ALIAS_VERSION.update()?;
    }
    Ok(())
}

fn fzf_url() -> String {
    let ver = metadata::fzf::VERSION;
    let arch = if_arm!("arm64", else "amd64");
    format!("https://github.com/junegunn/fzf/releases/download/v{ver}/fzf-{ver}-windows_{arch}.zip")
}

fn jq_url() -> String {
    let ver = metadata::jq::VERSION;
    format!("https://github.com/jqlang/jq/releases/download/jq-{ver}/jq-windows-amd64.exe")
}
