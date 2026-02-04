// This is free and unencumbered software released into the public domain.

use asimov_core::ModuleName;
use camino::Utf8PathBuf;
use std::io::Result;
use tokio::fs::ReadDir;

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
                && entry_type.is_symlink()
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
