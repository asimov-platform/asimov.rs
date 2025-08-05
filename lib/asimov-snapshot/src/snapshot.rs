// This is free and unencumbered software released into the public domain.

use asimov_module::resolve::Resolver;
use asimov_runner::{FetcherOptions, GraphOutput};
use jiff::{Span, Timestamp};
use std::{io::Result, string::String, vec::Vec};

#[derive(Clone, Debug, Default, bon::Builder)]
pub struct Options {
    /// Controls maximum age of the "current" snapshot that is allowed to be
    /// returned from `Snapshotter.read_current`.
    #[builder(with = |duration: std::time::Duration| -> core::result::Result<_, jiff::Error> { Span::try_from(duration) })]
    pub max_current_age: Option<Span>,
}

pub struct Snapshotter<S> {
    // TODO: not critical but would be nice to have the ability to inject the fetcher impl:
    // fetcher: F,
    resolver: Resolver,
    storage: S,
    options: Options,
}

impl<S> Snapshotter<S> {
    pub fn new(resolver: Resolver, storage: S, options: Options) -> Self {
        Self {
            resolver,
            storage,
            options,
        }
    }
}

impl<S: crate::storage::Storage> Snapshotter<S> {
    /// Fetches the content from an URL and saves it to the snapshot storage.
    #[tracing::instrument(skip(self), fields(url = url.as_ref()))]
    pub async fn snapshot(&mut self, url: impl AsRef<str>) -> Result<()> {
        let module = self
            .resolver
            .resolve(url.as_ref())
            .map_err(std::io::Error::other)?
            .first()
            .cloned()
            .ok_or_else(|| std::io::Error::other("No module found for fetch operation"))?;
        let timestamp = Timestamp::now();
        let program = std::format!("asimov-{}-fetcher", module.name);
        let options = FetcherOptions::builder().build();
        let data =
            asimov_runner::Fetcher::new(program, url.as_ref(), GraphOutput::Captured, options)
                .execute()
                .await
                .map_err(|e| std::io::Error::other(std::format!("Execution error: {e}")))?;

        self.storage.save(url, timestamp, data.get_ref())
    }

    /// Returns the snapshot content of an URL at the given timestamp.
    #[tracing::instrument(skip(self), fields(url = url.as_ref()))]
    pub async fn read(&self, url: impl AsRef<str>, timestamp: Timestamp) -> Result<Vec<u8>> {
        self.storage.read(url, timestamp)
    }

    /// Returns the latest snapshot content of an URL.
    ///
    /// A fresh snapshot is first created if [`Options::max_current_age`]
    /// is set and the latest snapshot is older than the maximum age.
    #[tracing::instrument(skip(self), fields(url = url.as_ref()))]
    pub async fn read_current(&mut self, url: impl AsRef<str>) -> Result<Vec<u8>> {
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
                    self.snapshot(&url).await?
                },
                Ok(std::cmp::Ordering::Less) => (),
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
