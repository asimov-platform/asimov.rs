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
                && (entry_type.is_file() || entry_type.is_symlink())
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
                && (entry_type.is_file() || entry_type.is_symlink())
                && let Some(ext) = std::path::Path::new(entry_name)
                    .extension()
                    .and_then(|ext| ext.to_str())
                && (ext == "json" || ext == "yaml" || ext == "yml")
                && let Ok(content) = tokio::fs::read(entry.path()).await
                && let Some(manifest) = match ext {
                    "json" => serde_json::from_slice(&content).ok(),
                    "yaml" | "yml" => serde_yaml_ng::from_slice(&content).ok(),
                    _ => None,
                }
            {
                return Some(manifest);
            }
        }
        None
    }
}
