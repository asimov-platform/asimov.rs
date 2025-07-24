// This is free and unencumbered software released into the public domain.

use crate::models::ModuleManifest;

use super::error::{DownloadError, FetchChecksumError, FetchError, HttpError, VerifyChecksumError};
use serde::Deserialize;
use std::{
    borrow::ToOwned as _,
    format,
    path::Path,
    string::{String, ToString as _},
    vec::Vec,
};
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};

#[derive(Debug, Deserialize)]
pub struct GitHubRelease {
    pub name: String,
    pub assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
}

pub async fn fetch_release(
    client: &reqwest::Client,
    module_name: &str,
    version: &str,
) -> Result<GitHubRelease, FetchError> {
    let url = format!(
        "https://api.github.com/repos/asimov-modules/asimov-{}-module/releases/tags/{}",
        module_name, version
    );

    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        Err(HttpError::NotSuccess(response.status()))?;
    }

    let content = response.bytes().await?;

    serde_json::from_slice::<'_, GitHubRelease>(&content)
        .map_err(|e| FetchError::Deserialize(e.into()))
}

pub async fn fetch_latest_release(
    client: &reqwest::Client,
    module_name: impl AsRef<str>,
) -> Result<String, FetchError> {
    let url = format!(
        "https://api.github.com/repos/asimov-modules/asimov-{}-module/releases/latest",
        module_name.as_ref()
    );

    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        Err(HttpError::NotSuccess(response.status()))?;
    }

    let content = response.bytes().await?;

    serde_json::from_slice::<'_, GitHubRelease>(&content)
        .map_err(|e| FetchError::Deserialize(e.into()))
        .map(|release| release.name)
}

pub async fn fetch_module_manifest(
    client: &reqwest::Client,
    module_name: &str,
    version: &str,
) -> Result<ModuleManifest, FetchError> {
    let url = format!(
        "https://raw.githubusercontent.com/asimov-modules/asimov-{}-module/{}/.asimov/module.yaml",
        module_name, version
    );

    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        Err(HttpError::NotSuccess(response.status()))?;
    }

    let content = response.bytes().await?;

    serde_yml::from_slice(&content).map_err(|e| FetchError::Deserialize(e.into()))
}

pub async fn fetch_checksum(
    client: &reqwest::Client,
    asset: &GitHubAsset,
) -> Result<Option<String>, FetchChecksumError> {
    let checksum_url = format!("{}.sha256", asset.browser_download_url);

    let response = client.get(&checksum_url).send().await?;

    if response.status() == 404 {
        return Ok(None);
    }

    if !response.status().is_success() {
        Err(HttpError::NotSuccess(response.status()))?;
    }

    Ok(Some(response.text().await?.trim().to_string()))
}

pub async fn verify_checksum(
    binary_path: &Path,
    expected_checksum: &str,
) -> Result<(), VerifyChecksumError> {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    let mut file = tokio::fs::File::open(binary_path).await?;
    let mut buf = std::vec![0u8; 10 * 1024];

    loop {
        let n = file.read(&mut buf).await?;
        if n == 0 {
            break; // End of file
        }
        hasher.update(&buf[..n]);
    }

    let actual_checksum = format!("{:x}", hasher.finalize());

    // Extract just the hash part from expected (in case it has filename)
    let expected_hash = expected_checksum
        .split_whitespace()
        .next()
        .unwrap_or(expected_checksum);

    if actual_checksum != expected_hash {
        return Err(VerifyChecksumError::InvalidChecksum(
            actual_checksum,
            expected_checksum.into(),
        ));
    }

    Ok(())
}

pub fn find_matching_asset<'a>(
    assets: &'a [GitHubAsset],
    module_name: &str,
    platform: &super::platform::PlatformInfo,
) -> Option<&'a GitHubAsset> {
    let patterns = if let Some(libc) = &platform.libc {
        std::vec![
            format!(
                "asimov-{}-module-{}-{}-{}.tar.gz",
                module_name, platform.os, platform.arch, libc
            ),
            format!(
                "asimov-{}-module-{}-{}-{}.zip",
                module_name, platform.os, platform.arch, libc
            ),
            format!(
                "asimov-{}-module-{}-{}.tar.gz",
                module_name, platform.os, platform.arch
            ),
            format!(
                "asimov-{}-module-{}-{}.zip",
                module_name, platform.os, platform.arch
            ),
        ]
    } else {
        std::vec![
            format!(
                "asimov-{}-module-{}-{}.tar.gz",
                module_name, platform.os, platform.arch
            ),
            format!(
                "asimov-{}-module-{}-{}.zip",
                module_name, platform.os, platform.arch
            ),
        ]
    };

    for pattern in patterns {
        if let Some(asset) = assets.iter().find(|asset| asset.name == pattern) {
            return Some(asset);
        }
    }

    None
}

pub async fn download_asset(
    client: &reqwest::Client,
    asset: &GitHubAsset,
    dst_dir: &Path,
) -> Result<std::path::PathBuf, DownloadError> {
    let mut response = client.get(&asset.browser_download_url).send().await?;

    if !response.status().is_success() {
        Err(HttpError::NotSuccess(response.status()))?;
    }

    let asset_path = dst_dir.join(&asset.name);
    let mut dst = tokio::fs::File::create(&asset_path).await?;

    while let Some(chunk) = response.chunk().await? {
        dst.write_all(&chunk).await?;
    }
    dst.flush().await?;

    Ok(asset_path)
}

pub async fn install_binaries(
    install_dir: &Path,
    src_asset: &Path,
) -> Result<(), tokio::io::Error> {
    tokio::task::spawn_blocking({
        let src_asset = src_asset.to_owned();
        let src_name = src_asset.to_string_lossy().into_owned();
        let dst = install_dir.clone();
        use std::io::{Error, Result};
        move || -> Result<()> {
            let asset_file = std::fs::File::open(&src_asset)?;
            if src_name.ends_with(".tar.gz") {
                let gz = flate2::read::GzDecoder::new(asset_file);
                let mut archive = tar::Archive::new(gz);
                archive.unpack(&dst)?;
            } else if src_name.ends_with(".zip") {
                let mut archive = zip::ZipArchive::new(asset_file)?;
                archive.extract(&dst)?;
            } else {
                return Err(Error::other("Unsupported format"));
            }
            Ok(())
        }
    })
    .await??;

    let mut read_dir = tokio::fs::read_dir(&temp_extract_dir).await?;

    while let Some(entry) = read_dir.next_entry().await? {
        if !entry.file_type().await?.is_file() {
            continue;
        }
        let name = entry.file_name();
        let mut src = tokio::fs::File::open(entry.path()).await?;
        let dst_path = install_dir.join(&name);
        let mut dst = tokio::fs::File::create(&dst_path).await?;
        tokio::io::copy(&mut src, &mut dst).await?;

        #[cfg(unix)]
        {
            use std::fs::Permissions;
            use std::os::unix::fs::PermissionsExt;
            tokio::fs::set_permissions(&dst_path, Permissions::from_mode(0o755)).await?;
        }
    }

    Ok(())
}
