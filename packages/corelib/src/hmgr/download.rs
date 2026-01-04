use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, OnceLock};
use std::time::Duration;

use cu::pre::*;
use reqwest::Client;

use crate::{opfs, hmgr};

static CLIENT: LazyLock<Result<Client, String>> = LazyLock::new(|| {
    Client::builder().gzip(true).https_only(true).build().map_err(|x| format!("{x}"))
});

fn client() -> cu::Result<&'static Client> {
    let client: &Result<Client, String> = &CLIENT;
    match client {
        Ok(c) => return Ok(c),
        Err(e) => {
            cu::bail!("error initializing https client: {e}")
        }
    }
}

/// Download URL to a temporary location, return the path to the downloaded file.
///
/// The SHA256 checksum is used to verify integrity.
///
/// The result is cached across multiple runs
pub fn download_file(
    identifier: &Path,
    url: &str,
    sha256_checksum: &str
) -> cu::Result<PathBuf> {
    cu::debug!("looking for download: {} from {}", identifier.display(), url);
    let target_path = hmgr::paths::download_file(identifier, url);
    let sha256_checksum = sha256_checksum.to_ascii_lowercase();
    if target_path.exists() {
        let actual_checksum = opfs::file_sha256(&target_path)?;
        if sha256_checksum == actual_checksum {
            cu::info!("got file from cache: {} ({})", identifier.display(), sha256_checksum);
            return Ok(target_path);
        }
    }

    for i in 0.. 5 {
        match i {
            0 => cu::info!("downloading {} from {}", identifier.display(), url),
            x => {
                cu::warn!("waiting for 5s before retrying...");
                std::thread::sleep(Duration::from_secs(5));
                cu::info!("downloading {} from {} (retry #{})", identifier.display(), url, x)
            }
        }
        let path = target_path.clone();
        let url = url.to_string();
        let result = cu::co::run(async move {
            do_download_file(path, url).await
        });
        if let Err(e) = result {
            cu::warn!("failed to download {}: {:?}", identifier.display(), e);
            continue;
        }
        let actual_checksum = match opfs::file_sha256(&target_path) {
            Err(e) => {
                cu::warn!("failed to hash {}: {:?}", identifier.display(), e);
                continue;
            }
            Ok(x) => x,
        };
        if sha256_checksum == actual_checksum {
            cu::info!("downloaded {} ({})", identifier.display(), sha256_checksum);
            return Ok(target_path);
        }
    }
    cu::bail!("failed to download {}", identifier.display());
}

async fn do_download_file(
    path: PathBuf,
    url: String,
) -> cu::Result<()> {
    let mut response = client()?.get(url).send().await?;
    let mut writer = cu::fs::buf_writer(path)?;

    while let Some(chunk) = response.chunk().await? {
        writer.write_all(&chunk)?;
    }
    writer.flush()?;
    Ok(())
}
