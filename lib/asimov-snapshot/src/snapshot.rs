// This is free and unencumbered software released into the public domain.

use asimov_module::resolve::Resolver;
use asimov_registry::Registry;
use asimov_runner::GraphOutput;
use jiff::{Span, Timestamp, ToSpan};
use std::{
    io::{self, Result},
    string::{String, ToString},
    vec::Vec,
};

use crate::Snapshot;

#[derive(Clone, Debug, bon::Builder)]
pub struct Options {
    /// Controls maximum age of the "current" snapshot that is allowed to be
    /// returned from `Snapshotter.read_current`.
    #[builder(required, default = Some(1.minute()))]
    #[builder(with = |duration: std::time::Duration| -> core::result::Result<_, jiff::Error> { Span::try_from(duration).map(Option::Some) })]
    pub max_current_age: Option<Span>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            max_current_age: Some(1.minute()),
        }
    }
}

pub struct Snapshotter<S> {
    // TODO: not critical but would be nice to have the ability to inject the fetcher impl:
    // fetcher: F,
    registry: Registry,
    storage: S,
    options: Options,

    cached_resolver: Option<Resolver>,
}

impl<S> Snapshotter<S> {
    pub fn new(registry: Registry, storage: S, options: Options) -> Self {
        Self {
            registry,
            storage,
            options,
            cached_resolver: None,
        }
    }
}

impl<S: crate::storage::Storage> Snapshotter<S> {
    /// Fetches the content from an URL and saves it to the snapshot storage.
    #[tracing::instrument(skip(self), fields(url = url.as_ref()))]
    pub async fn snapshot(&mut self, url: impl AsRef<str>) -> Result<Snapshot> {
        if self.cached_resolver.is_none() {
            let modules = self
                .registry
                .enabled_modules()
                .await
                .map_err(io::Error::other)?
                .into_iter()
                .map(|enabled| enabled.manifest)
                .filter(|manifest| {
                    manifest
                        .provides
                        .programs
                        .iter()
                        .any(|p| p.ends_with("-fetcher") || p.ends_with("-cataloger"))
                });
            let resolver = Resolver::try_from_iter(modules).map_err(io::Error::other)?;
            self.cached_resolver = Some(resolver);
        }
        let resolver = self.cached_resolver.as_ref().unwrap();

        let module = resolver
            .resolve(url.as_ref())
            .map_err(std::io::Error::other)?
            .first()
            .cloned()
            .ok_or_else(|| std::io::Error::other("No module found for creating snapshot"))?;

        let programs = self
            .registry
            .read_manifest(&module.name)
            .await
            .map_err(io::Error::other)?
            .manifest
            .provides
            .programs;

        let url = url.as_ref().to_string();

        let start_timestamp = Timestamp::now();

        let data = if let Some(program) = programs.iter().find(|p| p.ends_with("-fetcher")) {
            asimov_runner::Fetcher::new(program, &url, GraphOutput::Captured, Default::default())
                .execute()
                .await
        } else if let Some(program) = programs.iter().find(|p| p.ends_with("-cataloger")) {
            asimov_runner::Cataloger::new(program, &url, GraphOutput::Captured, Default::default())
                .execute()
                .await
        } else {
            return Err(std::io::Error::other(
                "No module found for creating snapshot",
            ));
        }
        .map_err(|e| std::io::Error::other(std::format!("Execution error: {e}")))?
        .into_inner(); // TODO: consider using the std::io::Cursor?

        let end_timestamp = Some(Timestamp::now());

        let snapshot = Snapshot {
            url,
            start_timestamp,
            end_timestamp,
            data,
        };

        self.storage.save(&snapshot)?;

        Ok(snapshot)
    }

    /// Returns the snapshot content of an URL at the given timestamp.
    #[tracing::instrument(skip(self), fields(url = url.as_ref()))]
    pub async fn read(&self, url: impl AsRef<str>, timestamp: Timestamp) -> Result<Snapshot> {
        self.storage.read(url, timestamp)
    }

    /// Returns the latest snapshot content of an URL.
    ///
    /// A fresh snapshot is first created if [`Options::max_current_age`]
    /// is set and the latest snapshot is older than the maximum age.
    #[tracing::instrument(skip(self), fields(url = url.as_ref()))]
    pub async fn read_current(&mut self, url: impl AsRef<str>) -> Result<Snapshot> {
        if let Some(max_age) = &self.options.max_current_age {
            let ts = self
                .storage
                .current_version(&url)?
                .to_zoned(jiff::tz::TimeZone::UTC);
            let now = jiff::Zoned::now();

            let diff = &now - &ts;

            // should never error, we provide `(jiff::Span, &jiff::Zoned)` to `compare`.
            // doc:
            // > If either of the spans being compared have a non-zero calendar
            // > unit (units bigger than hours), then this routine requires a
            // > relative datetime. If one is not provided, then an error is
            // > returned.
            match diff.compare((max_age, &now)) {
                Ok(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal) => {
                    tracing::debug!("Updating current snapshot...");
                    return self.snapshot(&url).await;
                },
                Ok(std::cmp::Ordering::Less) => return self.storage.read_current(url),
                Err(err) => {
                    tracing::error!(?err, "unable to compare timestamps, not updating snapshot")
                },
            };
        };

        self.storage.read_current(url)
    }

    /// Returns the list of snapshotted URLs along with their latest timestamps.
    /// Entries can be read by [`Self::read`].
    #[tracing::instrument(skip(self))]
    pub async fn list(&self) -> Result<Vec<(String, Timestamp)>> {
        self.storage.list_urls()
    }

    /// Returns the log of timestamps for a given URL.
    /// Entries can be read by [`Self::read`].
    #[tracing::instrument(skip(self))]
    pub async fn log(&self, url: &str) -> Result<Vec<Timestamp>> {
        self.storage.list_snapshots(url)
    }

    /// Deletes old snapshots for a given URL.
    /// Currently everything but the latest snapshot is deleted.
    #[tracing::instrument(skip(self), fields(url = url.as_ref()))]
    pub async fn compact(&self, url: impl AsRef<str>) -> Result<()> {
        // TODO: max hourly/daily/weekly/monthly/yearly snapshots
        let timestamps = self.storage.list_snapshots(&url)?;
        let Some(latest) = timestamps.iter().max() else {
            return Ok(());
        };
        tracing::debug!("Deleting snapshots older than `{latest}`");
        for &ts in timestamps.iter().filter(|&ts| ts != latest) {
            self.storage.delete(&url, ts)?;
        }
        Ok(())
    }
}
