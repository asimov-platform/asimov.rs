// This is free and unencumbered software released into the public domain.

use crate::Progress;
use anyhow::{anyhow, Result};
use hf_hub::{api::sync::ApiBuilder, Cache, Repo, RepoType};
use std::path::PathBuf;

/// Ensure that a specific file exists locally.
/// Uses cache first; downloads with progress if missing.
pub fn ensure_file(repo: &str, filename: &str) -> Result<PathBuf> {
    let api = ApiBuilder::from_env().build()?;
    let cache = Cache::default();
    let repo_id = Repo::new(repo.to_owned(), RepoType::Model);

    if let Some(p) = cache.repo(repo_id.clone()).get(filename) {
        return Ok(p);
    }

    let progress = Progress::global().clone();
    let path = api.repo(repo_id).download_with_progress(filename, progress)?;
    Ok(path)
}

/// Ensure that an entire model snapshot is available locally.
/// Returns the directory containing the snapshot.
pub fn ensure_snapshot(repo: &str, revision: Option<&str>) -> Result<PathBuf> {
    let api = ApiBuilder::from_env().build()?;
    let cache = Cache::default();
    let repo_id = Repo::new(repo.to_owned(), RepoType::Model);

    let repo_api = if let Some(rev) = revision {
        api.repo(Repo::with_revision(repo.to_owned(), RepoType::Model, rev.to_owned()))
    } else {
        api.repo(repo_id.clone())
    };

    let info = repo_api.info()?;
    let mut first_file_path: Option<PathBuf> = None;

    for s in info.siblings {
        if let Some(p) = cache.repo(repo_id.clone()).get(&s.rfilename) {
            if first_file_path.is_none() {
                first_file_path = Some(p);
            }
            continue;
        }

        let progress = Progress::global().clone();
        let p = repo_api.download_with_progress(&s.rfilename, progress)?;
        if first_file_path.is_none() {
            first_file_path = Some(p);
        }
    }

    let snapshot_dir = first_file_path
        .and_then(|p| p.parent().map(|pp| pp.to_path_buf()))
        .ok_or_else(|| anyhow!("snapshot is empty"))?;

    Ok(snapshot_dir)
}
