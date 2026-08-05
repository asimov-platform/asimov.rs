// This is free and unencumbered software released into the public domain.

use asimov_core::ModuleName;
use asimov_module::InstalledModuleManifest;
use camino::Utf8PathBuf;
use std::io::Result;
use tokio::fs::ReadDir;

/// An iterator over module names in a module directory.
#[derive(Debug)]
pub struct ModuleNameIterator {
    dir: ReadDir,
}

impl ModuleNameIterator {
    pub async fn new(path: Utf8PathBuf) -> Result<Self> {
        Ok(ModuleNameIterator {
            dir: tokio::fs::read_dir(path).await?,
        })
    }
}

impl crate::ModuleNameIterator for ModuleNameIterator {
    async fn next(&mut self) -> Option<ModuleName> {
        while let Some(entry) = self.dir.next_entry().await.transpose() {
            if let Ok(entry) = entry
                && let Some(entry_name) = entry.file_name().to_str()
                && !entry_name.starts_with(".")
                && let Ok(entry_type) = entry.file_type().await
                && (entry_type.is_dir() || entry_type.is_file() || entry_type.is_symlink())
            {
                let entry_stem = [".json", ".yaml"]
                    .iter()
                    .find_map(|&ext| entry_name.strip_suffix(ext))
                    .unwrap_or(entry_name);
                return Some(entry_stem.into());
            }
        }
        None
    }
}

/// An iterator over module manifests in a module directory.
#[derive(Debug)]
pub struct ModuleManifestIterator {
    dir: ReadDir,
}

impl ModuleManifestIterator {
    pub async fn new(path: Utf8PathBuf) -> Result<Self> {
        Ok(ModuleManifestIterator {
            dir: tokio::fs::read_dir(path).await?,
        })
    }
}

impl crate::ModuleManifestIterator for ModuleManifestIterator {
    async fn next(&mut self) -> Option<InstalledModuleManifest> {
        while let Some(entry) = self.dir.next_entry().await.transpose() {
            if let Ok(entry) = entry
                && let Some(entry_name) = entry.file_name().to_str()
                && !entry_name.starts_with(".")
                && let Ok(entry_type) = entry.file_type().await
            {
                let manifest = if entry_type.is_dir() {
                    let dir = entry.path();
                    let mut manifest = None;
                    for file in ["manifest.json", "manifest.yaml", "manifest.yml"] {
                        manifest = read_manifest(&dir.join(file)).await;
                        if manifest.is_some() {
                            break;
                        }
                    }
                    manifest
                } else if entry_type.is_file() || entry_type.is_symlink() {
                    read_manifest(&entry.path()).await
                } else {
                    None
                };

                if let Some(manifest) = manifest {
                    return Some(manifest);
                }
            }
        }
        None
    }
}

async fn read_manifest(path: &std::path::Path) -> Option<InstalledModuleManifest> {
    let content = tokio::fs::read(path).await.ok()?;

    match path.extension().and_then(|ext| ext.to_str())? {
        "json" => serde_json::from_slice(&content).ok(),
        "yaml" | "yml" => serde_yaml_ng::from_slice(&content).ok(),
        _ => None,
    }
}
