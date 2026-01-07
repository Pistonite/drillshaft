use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static HOME_PATH: OnceLock<PathBuf> = OnceLock::new();

/// Initialize the SHAFT_HOME directory path.
///
/// Will fail silently and print a warning if it's already set
pub fn init_home_path(path: PathBuf) {
    cu::debug!("initializing home path: {}", path.display());
    if HOME_PATH.set(path).is_err() {
        cu::warn!(
            "SHAFT_HOME is already initialized at '{}'",
            HOME_PATH.get().unwrap().display()
        )
    }
}

fn home() -> &'static Path {
    HOME_PATH
        .get()
        .expect("home not initialized; please debug with -vv")
}

/// HOME/install_cache.json
#[inline(always)]
pub fn install_cache_json() -> PathBuf {
    home().join("install_cache.json")
}

/// HOME/shaft or HOME/shaft.exe
#[inline(always)]
pub fn shaft_binary() -> PathBuf {
    home().join(crate::bin_name!("shaft"))
}

/// HOME/shaft.old or HOME/shaft.old.exe
#[inline(always)]
pub fn shaft_binary_old() -> PathBuf {
    home().join(crate::bin_name!("shaft.old"))
}

/// HOME/environment.json
#[inline(always)]
pub fn environment_json() -> PathBuf {
    home().join("environment.json")
}

/// HOME/previous_command.json
#[inline(always)]
pub fn previous_command_json() -> PathBuf {
    home().join("previous_command.json")
}

/// HOME/version_cache.json
#[inline(always)]
pub fn version_cache_json() -> PathBuf {
    home().join("version_cache.json")
}

/// HOME/config.toml
#[inline(always)]
pub fn config_toml() -> PathBuf {
    home().join("config.toml")
}

/// HOME/.interruped
#[inline(always)]
pub fn dot_interrupted() -> PathBuf {
    home().join(".interrupted")
}

/// HOME/.lock
#[inline(always)]
pub fn dot_lock() -> PathBuf {
    home().join(".lock")
}

/// HOME/windows-shell
#[inline(always)]
pub fn windows_shell_root() -> PathBuf {
    home().join("windows-shell")
}

/// HOME/init/
#[inline(always)]
pub fn init_root() -> PathBuf {
    home().join("init")
}

/// HOME/bin/
#[inline(always)]
pub fn bin_root() -> PathBuf {
    home().join("bin")
}

/// HOME/bin/<binary>
#[inline(always)]
pub fn binary(file: impl AsRef<Path>) -> PathBuf {
    let mut bin = bin_root();
    bin.push(file);
    bin
}

/// HOME/temp/
#[inline(always)]
pub fn temp_root() -> PathBuf {
    home().join("temp")
}

/// HOME/temp/<package>
#[inline(always)]
pub fn temp_dir(package: impl AsRef<Path>) -> PathBuf {
    let mut x = temp_root();
    x.push(package);
    x
}

/// HOME/install/
#[inline(always)]
pub fn install_root() -> PathBuf {
    home().join("install")
}

/// HOME/install/<package>
#[inline(always)]
pub fn install_dir(package: impl AsRef<Path>) -> PathBuf {
    let mut x = install_root();
    x.push(package);
    x
}

/// HOME/install-old/
#[inline(always)]
pub fn install_old_root() -> PathBuf {
    home().join("install-old")
}

/// HOME/install-old/<package>
#[inline(always)]
pub fn install_old_dir(package: impl AsRef<Path>) -> PathBuf {
    let mut x = install_old_root();
    x.push(package);
    x
}

/// HOME/download/
#[inline(always)]
pub fn download_root() -> PathBuf {
    home().join("download")
}

/// HOME/download/<identifier_stem>-<url_hash>.<ext>
#[inline(always)]
pub fn download(identifier: impl AsRef<Path>, url: impl AsRef<str>) -> PathBuf {
    download_file_impl(identifier.as_ref(), url.as_ref())
}
fn download_file_impl(identifier: &Path, url: &str) -> PathBuf {
    let hash = fxhash::hash64(url);
    let mut path_part = OsString::new();
    if let Some(stem) = identifier.file_stem() {
        path_part.push(stem);
        path_part.push("-");
    }
    path_part.push(format!("{hash:016x}"));
    if let Some(ext) = identifier.extension() {
        path_part.push(".");
        path_part.push(ext);
    }
    let mut path = download_root();
    path.push(path_part);
    path
}

#[inline(always)]
pub fn clean_temp_dir(package: impl AsRef<Path>) {
    clean_temp_dir_impl(package.as_ref())
}
fn clean_temp_dir_impl(package: &Path) {
    if let Err(e) = cu::fs::rec_remove(temp_dir(package)) {
        cu::warn!("failed to remove temp dir: {e:?}");
    }
}
