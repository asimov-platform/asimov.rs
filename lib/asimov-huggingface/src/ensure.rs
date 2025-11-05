// This is free and unencumbered software released into the public domain.

//! Cache-aware download helpers that always use the unified progress bar.

use crate::{HuggingfaceError, Progress, Result};
use hf_hub::{Cache, Repo, RepoType, api::sync::ApiBuilder};
use std::path::PathBuf;

/// Checks whether the specified file from a Hugging Face repository exists locally.
///
/// Arguments:
/// - `repo`: repository id, e.g. `"facebook/dinov2-base"`.
/// - `filename`: file within the repository, e.g. `"pytorch_model.bin"`.
pub fn file_exists(repo: &str, filename: &str) -> Option<PathBuf> {
    let cache = Cache::default();
    let repo_id = Repo::new(repo.to_owned(), RepoType::Model);
    cache.repo(repo_id).get(filename)
}

/// Ensures that the specified file from a Hugging Face repository is available locally.
///
/// Behavior:
/// - Checks the local Hugging Face cache first.
/// - If the file is missing, downloads it and displays a unified progress bar.
/// - Returns the absolute path to the cached file.
///
/// Arguments:
/// - `repo`: repository id, e.g. `"facebook/dinov2-base"`.
/// - `filename`: file within the repository, e.g. `"pytorch_model.bin"`.
///
/// Errors:
/// - Propagates Hugging Face API errors (`HuggingfaceError::Api`).
pub fn ensure_file(repo: &str, filename: &str) -> Result<PathBuf> {
    let api = ApiBuilder::from_env().build()?;
    let cache = Cache::default();
    let repo_id = Repo::new(repo.to_owned(), RepoType::Model);

    if let Some(path) = cache.repo(repo_id.clone()).get(filename) {
        return Ok(path);
    }

    let progress = Progress::new();
    let path = api
        .repo(repo_id)
        .download_with_progress(filename, progress)?;
    Ok(path)
}

/// Ensures that a complete snapshot of a Hugging Face repository is present locally
/// and returns the snapshot directory path.
///
/// Behavior:
/// - Queries repository contents and attempts to resolve all files in the snapshot.
/// - Reuses files already present in the local cache.
/// - Downloads missing files with a unified progress bar.
/// - Returns the directory that contains the resolved snapshot files.
///
/// Arguments:
/// - `repo`: repository id, e.g. `"julien-c/dummy-unknown"`.
/// - `revision`: optional revision (commit hash, tag, or branch). If `None`, uses the default branch.
///
/// Errors:
/// - `HuggingfaceError::Api` on Hugging Face API failures.
/// - `HuggingfaceError::EmptySnapshot` if no files were resolved.
pub fn ensure_snapshot(repo: &str, revision: Option<&str>) -> Result<PathBuf> {
    let api = ApiBuilder::from_env().build()?;
    let cache = Cache::default();
    let repo_id = Repo::new(repo.to_owned(), RepoType::Model);

    let repo_api = if let Some(rev) = revision {
        api.repo(Repo::with_revision(
            repo.to_owned(),
            RepoType::Model,
            rev.to_owned(),
        ))
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

        let progress = Progress::new();
        let p = repo_api.download_with_progress(&s.rfilename, progress)?;
        if first_file_path.is_none() {
            first_file_path = Some(p);
        }
    }

    let snapshot_dir = first_file_path
        .and_then(|p| p.parent().map(|pp| pp.to_path_buf()))
        .ok_or(HuggingfaceError::EmptySnapshot)?;

    Ok(snapshot_dir)
}
