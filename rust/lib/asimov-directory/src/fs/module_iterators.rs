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
                && (entry_type.is_dir() || entry_type.is_symlink())
                && let Ok(module_name) = ModuleName::try_from(entry_name)
            {
                return Some(module_name);
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
                && (entry_type.is_dir() || entry_type.is_symlink())
                && let Ok(content) = tokio::fs::read(entry.path().join("manifest.json")).await
                && let Ok(manifest) = serde_json::from_slice(&content)
            {
                return Some(manifest);
            }
        }
        None
    }
}
