use std::io::Read;
use std::path::Path;

use cu::pre::*;
use sha2::{Digest, Sha256};

use crate::opfs;

/// Create a Windows symbolic link (requires sudo).
/// `from` is where the link will be
#[inline(always)]
pub fn symlink_file(from: impl AsRef<Path>, to: impl AsRef<Path>) -> cu::Result<()> {
    symlink_file_impl(from.as_ref(), to.as_ref())
}
#[cfg(windows)]
#[cu::error_ctx("failed to create symbolic link from '{}' to '{}'", from.display(), to.display())]
fn symlink_file_impl(from: &Path, to: &Path) -> cu::Result<()> {
    if from.exists() {
        cu::bail!("the symlink already exists");
    }
    let from_abs = from.normalize()?;
    let to_abs = to.normalize()?;
    let from_str = from_abs.as_utf8()?;
    let to_str = to_abs.as_utf8()?;

    // use powershell since sudo is required
    cu::debug!("creating symlink from '{from_str}' to '{to_str}'");
    let script =
        format!("New-Item -ItemType SymbolicLink -Path \"{from_str}\" -Target \"{to_str}\"");

    opfs::sudo("powershell", "create symlink")?
        .args(["-NoLogo", "-NoProfile", "-c", &script])
        .stdout(cu::lv::D)
        .stderr(cu::lv::E)
        .stdin_null()
        .wait_nz()?;

    Ok(())
}

/// Get the SHA256 checksum of a file and return it as a string
#[cu::error_ctx("failed to hash file: '{}'", path.display())]
pub fn file_sha256(path: &Path) -> cu::Result<String> {
    let mut hasher = Sha256::new();
    let mut reader = cu::fs::reader(&path)?;
    let mut buf = vec![0u8; 409600].into_boxed_slice();
    loop {
        let i = reader.read(&mut buf)?;
        if i == 0 {
            break;
        }
        hasher.update(&buf[..i]);
    }
    let result = hasher.finalize();
    let mut out = String::with_capacity(64);
    let digits = b"0123456789abcdef";
    for b in result {
        let c1 = digits[(b / 16) as usize] as char;
        let c2 = digits[(b % 16) as usize] as char;
        out.push(c1);
        out.push(c2);
    }
    Ok(out)
}

/// Extract an archive with 7z. Requires the 7z binary to exist
#[inline(always)]
pub fn un7z(archive_path: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> cu::Result<()> {
    un7z_impl(archive_path.as_ref(), out_dir.as_ref())
}

#[cu::error_ctx("failed to extract zip: '{}'", archive_path.display())]
fn un7z_impl(archive_path: &Path, out_dir: &Path) -> cu::Result<()> {
    let out_switch = format!("-o{}", quote_path(out_dir)?);
    // 7z will create the out dir if not exist, so we don't need to check
    cu::which("7z")?
        .command()
        .add(cu::args!["x", "-y", archive_path, out_switch])
        .stdoe(cu::lv::D)
        .stdin_null()
        .wait_nz()?;
    Ok(())
}

/// Ensure nothing weird happens when the path is quoted
#[inline(always)]
pub fn quote_path(path: impl AsRef<Path>) -> cu::Result<String> {
    quote_path_impl(path.as_ref())
}
fn quote_path_impl(path: &Path) -> cu::Result<String> {
    if cfg!(windows) {
        // quote cannot be in the path on Windows
        Ok(format!("\"{}\"", path.as_utf8()?))
    } else {
        let s = path.as_utf8()?;
        cu::ensure!(
            !s.contains('"'),
            "quote (\") in path is not allowed: {}",
            path.display()
        );
        Ok(format!("\"{s}\""))
    }
}
