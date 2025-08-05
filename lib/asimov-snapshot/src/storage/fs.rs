// This is free and unencumbered software released into the public domain.

use cap_std::fs::{Permissions, PermissionsExt as _};
use jiff::Timestamp;
use std::{
    format,
    io::{Result, Write},
    string::String,
    vec::Vec,
};

const TIMESTAMP_FORMAT_STRING: &str = "%Y%m%dT%H%M%SZ";

pub struct Fs {
    root: cap_std::fs::Dir,
}

impl Fs {
    pub fn new(root: cap_std::fs::Dir) -> Self {
        Self { root }
    }

    pub fn for_dir(path: std::path::PathBuf) -> Result<Self> {
        let root = cap_std::fs::Dir::open_ambient_dir(&path, cap_std::ambient_authority())?;
        Ok(Self::new(root))
    }
}

impl super::Storage for Fs {
    #[tracing::instrument(skip(self, data), fields(url = url.as_ref()))]
    fn save_timestamp(
        &self,
        url: impl AsRef<str>,
        timestamp: Timestamp,
        data: impl AsRef<[u8]>,
    ) -> Result<()> {
        let url_hash = hex::encode(sha256(url.as_ref()));
        let url_dir = std::path::Path::new(&url_hash);

        tracing::debug!(hash = url_hash, "Creating directory for url");
        self.root.create_dir_all(url_dir)?;

        let ts = timestamp.strftime(TIMESTAMP_FORMAT_STRING);
        let filename = format!("{ts}.jsonld");

        let snapshot_path = url_dir.join(filename);

        tracing::debug!("Writing snapshot");
        let mut snapshot_file = self.root.create(&snapshot_path)?;
        snapshot_file.write_all(data.as_ref())?;
        snapshot_file.set_permissions(Permissions::from_mode(0o444))?;

        let url_path = url_dir.join(".url");

        if !self
            .root
            .metadata(&url_path)
            .map(|m| m.is_file())
            .unwrap_or(false)
        {
            tracing::debug!(path = ?url_path, "Creating `url` metadata file");
            let mut url_file = self.root.create(&url_path)?;
            url_file.write_all(url.as_ref().as_bytes())?;
            url_file.set_permissions(Permissions::from_mode(0o444))?;
        }

        Ok(())
    }

    #[tracing::instrument(skip(self), fields(url = url.as_ref()))]
    fn read(&self, url: impl AsRef<str>, timestamp: Timestamp) -> Result<Vec<u8>> {
        let url_hash = hex::encode(sha256(url.as_ref()));

        let ts = timestamp.strftime(TIMESTAMP_FORMAT_STRING);
        let filename = format!("{ts}.jsonld");

        let file_path = std::path::Path::new(&url_hash).join(filename);

        tracing::debug!("Reading snapshot");
        self.root.read(file_path)
    }

    #[tracing::instrument(skip(self), fields(url = url.as_ref()))]
    fn set_current_version(&self, url: impl AsRef<str>, timestamp: Timestamp) -> Result<()> {
        let url_hash = hex::encode(sha256(url.as_ref()));
        let url_dir = std::path::Path::new(&url_hash);

        let current_link_path = url_dir.join("current");

        tracing::debug!(source = ?current_link_path, "Removing old `current` symlink");
        self.delete_current_version(&url)?;

        let ts = timestamp.strftime(TIMESTAMP_FORMAT_STRING);
        let snapshot_name = format!("{ts}.jsonld");

        tracing::debug!(source = ?current_link_path, target = ?snapshot_name, "Creating new `current` symlink");

        #[cfg(unix)]
        return self.root.symlink(&snapshot_name, current_link_path);

        #[cfg(windows)]
        return self.root.symlink_file(&snapshot_name, current_link_path);
    }

    #[tracing::instrument(skip(self), fields(url = url.as_ref()))]
    fn current_version(&self, url: impl AsRef<str>) -> Result<Timestamp> {
        let url_hash = hex::encode(sha256(url.as_ref()));

        let link_path = std::path::Path::new(&url_hash).join("current");

        tracing::debug!(path = ?link_path, "Reading `current` symlink");
        let current = self.root.read_link(link_path)?;

        let stem = current
            .file_stem()
            .ok_or_else(|| {
                std::io::Error::other(format!(
                    "Malformed file: `{}` does not have a valid file stem",
                    current.display()
                ))
            })?
            .to_string_lossy();

        stem.parse()
            .map_err(|e| std::io::Error::other(format!("Invalid timestamp `{stem}`: {e}")))
    }

    #[tracing::instrument(skip(self))]
    fn list_urls(&self) -> Result<Vec<(String, Timestamp)>> {
        let mut urls = Vec::new();

        let read_dir = self.root.read_dir("./")?;
        for entry in read_dir {
            let entry = entry?;
            let url_hash = entry.file_name();

            let url_link = std::path::Path::new(&url_hash).join(".url");

            tracing::debug!(hash = ?url_hash, path=?url_link, "Reading `url` metadata file");
            let url = match self.root.read_to_string(&url_link) {
                Ok(url) => url,
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,
                Err(err) => return Err(err),
            };

            let current = self.current_version(&url)?;

            urls.push((url, current));
        }

        Ok(urls)
    }

    #[tracing::instrument(skip(self), fields(url = url.as_ref()))]
    fn list_snapshots(&self, url: impl AsRef<str>) -> Result<Vec<Timestamp>> {
        let url_hash = hex::encode(sha256(url.as_ref()));
        let url_dir = std::path::Path::new(&url_hash);

        let mut tss = Vec::new();

        tracing::debug!("Reading directory");
        let read_dir = self.root.read_dir(url_dir)?;
        for entry in read_dir {
            let entry = entry?;

            let filename = entry.file_name();
            let filename = std::path::Path::new(&filename);

            let stem = filename
                .file_stem()
                .ok_or_else(|| {
                    std::io::Error::other(format!(
                        "Malformed file: `{}` does not have a valid file stem",
                        filename.display()
                    ))
                })?
                .to_string_lossy();

            if stem == "current" {
                continue;
            }
            if stem == ".url" {
                continue;
            }

            let ts: Timestamp = stem
                .parse()
                .map_err(|e| std::io::Error::other(format!("Invalid timestamp `{stem}`: {e}")))?;

            tss.push(ts)
        }

        Ok(tss)
    }

    #[tracing::instrument(skip(self), fields(url = url.as_ref()))]
    fn delete(&self, url: impl AsRef<str>, timestamp: Timestamp) -> Result<()> {
        let url_hash = hex::encode(sha256(url.as_ref()));
        let url_dir = std::path::Path::new(&url_hash);

        let ts = timestamp.strftime(TIMESTAMP_FORMAT_STRING);
        let filename = format!("{ts}.jsonld");

        let snapshot_path = url_dir.join(filename);

        tracing::debug!(path = ?snapshot_path, "Deleting snapshot");
        self.root.remove_file(&snapshot_path).or_else(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(e)
            }
        })?;

        let Ok(current) = self.current_version(&url) else {
            return Ok(());
        };

        if timestamp != current {
            return Ok(());
        }

        tracing::debug!("Deleted snapshot was `current`, searching for new `current`...");

        let versions = self.list_snapshots(&url)?;

        if let Some(new_current) = versions.into_iter().max() {
            tracing::debug!(?new_current, "Updating `current`");
            self.set_current_version(url, new_current)
        } else {
            tracing::debug!("Last snapshot deleted, removing `current`");
            self.delete_current_version(&url)
        }
    }
}

impl Fs {
    fn delete_current_version(&self, url: impl AsRef<str>) -> Result<()> {
        let url_hash = hex::encode(sha256(url.as_ref()));
        let url_dir = std::path::Path::new(&url_hash);

        let current_link_path = url_dir.join("current");

        self.root.remove_file(&current_link_path).or_else(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(e)
            }
        })
    }
}

fn sha256(data: impl AsRef<[u8]>) -> sha2::digest::Output<sha2::Sha256> {
    use sha2::Digest;
    sha2::Sha256::digest(data.as_ref())
}

#[cfg(test)]
mod tests {
    use jiff::{ToSpan, Unit};

    use super::*;
    use crate::storage::Storage;
    use std::{eprintln, io::Result, string::ToString};

    #[test]
    fn storage() -> Result<()> {
        tracing_subscriber::fmt::init();

        let tmp_dir = tempfile::tempdir()?;
        let tmp_dir = tmp_dir.path().join("asimov-snapshot-cli-test");
        let auth = cap_std::ambient_authority();
        cap_std::fs::Dir::create_ambient_dir_all(&tmp_dir, auth)?;
        let root = cap_std::fs::Dir::open_ambient_dir(&tmp_dir, auth)?;

        eprintln!("Testing directory: {tmp_dir:?}");

        let fs = Fs { root };

        let url = "http://example.org/";
        let first_ts = Timestamp::now().round(Unit::Second).unwrap();

        fs.save(url, first_ts, r"v1")?;

        let current = fs.current_version(url)?;
        assert_eq!(current, first_ts);

        let second_ts = Timestamp::now()
            .round(Unit::Second)
            .unwrap()
            .checked_sub(1.hour())
            .unwrap();

        fs.save(url, second_ts, r"v2")?;

        let current = fs.current_version(url)?;
        assert_eq!(
            current, first_ts,
            "Saving older timestamps should not affect the `current` link"
        );

        let third_ts = Timestamp::now()
            .round(Unit::Second)
            .unwrap()
            .checked_add(1.hour())
            .unwrap();

        fs.save(url, third_ts, r"v3")?;

        let current = fs.current_version(url)?;
        assert_eq!(
            current, third_ts,
            "Saving newer timestamps should update the `current` link"
        );

        let urls = fs.list_urls()?;
        assert_eq!(
            urls.as_slice(),
            &[(url.to_string(), third_ts)],
            "A single URL should be returned"
        );

        let snapshots = fs.list_snapshots(&url)?;
        assert_eq!(snapshots.len(), 3);
        assert!(snapshots.contains(&first_ts));
        assert!(snapshots.contains(&second_ts));
        assert!(snapshots.contains(&third_ts));

        fs.delete(&url, third_ts)?;
        assert_eq!(fs.list_snapshots(&url)?.len(), 2);
        assert_eq!(fs.current_version(&url)?, first_ts);

        fs.delete(&url, first_ts)?;
        assert_eq!(fs.current_version(&url)?, second_ts);
        assert_eq!(fs.list_snapshots(&url)?.len(), 1);

        fs.delete(&url, second_ts)?;
        assert_eq!(fs.list_snapshots(&url)?.len(), 0);

        assert_eq!(
            fs.current_version(&url).unwrap_err().kind(),
            std::io::ErrorKind::NotFound
        );

        Ok(())
    }
}
