// This is free and unencumbered software released into the public domain.

use asimov_env::paths::asimov_root;
use asimov_module::resolve::Resolver;
use asimov_runner::{FetcherOptions, GraphOutput};
use chrono::{DateTime, prelude::*};
use std::{io::Result, string::String, vec::Vec};

pub struct Snapshotter<S> {
    // TODO: not critical but would be nice to have the ability to inject the fetcher impl:
    // fetcher: F,
    resolver: Resolver,
    storage: S,
}

impl<S> Snapshotter<S> {
    pub fn new(resolver: Resolver, storage: S) -> Self {
        Self { resolver, storage }
    }

    pub fn new_fs() -> Result<Snapshotter<crate::storage::Fs>> {
        let snapshot_dir = asimov_root().join("snapshots");
        let storage = crate::storage::Fs::for_dir(snapshot_dir)?;
        let manifest_dir = asimov_root().join("modules");
        let resolver = Resolver::try_from_dir(manifest_dir).map_err(std::io::Error::other)?;
        Ok(Snapshotter { resolver, storage })
    }
}

impl<S: crate::storage::Storage> Snapshotter<S> {
    #[tracing::instrument(skip(self))]
    pub async fn snapshot(&mut self, url: &str) -> Result<()> {
        let module = self
            .resolver
            .resolve(url)
            .map_err(|e| std::io::Error::other(e))?
            .first()
            .cloned()
            .ok_or_else(|| std::io::Error::other("No module found for fetch operation"))?;
        let timestamp = Utc::now();
        let program = std::format!("asimov-{}-fetcher", module.name);
        let options = FetcherOptions::builder().output("jsonld").build();
        let data = asimov_runner::Fetcher::new(program, url, GraphOutput::Captured, options)
            .execute()
            .await
            .map_err(|e| std::io::Error::other(std::format!("Execution error: {e}")))?;

        self.storage.save(url, timestamp, data.get_ref())
    }

    #[tracing::instrument(skip(self))]
    pub async fn list(&self) -> Result<Vec<(String, DateTime<Utc>)>> {
        self.storage.list_urls()
    }

    #[tracing::instrument(skip(self))]
    pub async fn log(&self, url: &str) -> Result<Vec<DateTime<Utc>>> {
        self.storage.list_snapshots(url)
    }

    #[tracing::instrument(skip(self))]
    pub async fn compact(&self, url: &str) -> Result<()> {
        // TODO: max hourly/daily/weekly/monthly/yearly snapshots
        let timestamps = self.storage.list_snapshots(url)?;
        let Some(latest) = timestamps.iter().max() else {
            return Ok(());
        };
        tracing::debug!("Deleting snapshots older than `{latest}`");
        for &ts in timestamps.iter().filter(|&ts| ts != latest) {
            self.storage.delete(url, ts)?;
        }
        Ok(())
    }
}
