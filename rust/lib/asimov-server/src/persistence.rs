// This is free and unencumbered software released into the public domain.

use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    sync::{Arc, LazyLock, RwLock},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to read or write")]
    IoError(#[from] std::io::Error),
    #[error("failed to serialize or deserialize")]
    SerdeError(#[from] serde_json::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

static STATE: LazyLock<Arc<RwLock<PersistentState>>> =
    LazyLock::new(|| Arc::new(RwLock::new(read().unwrap_or_default())));

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PersistentState {
    pub provider: String,
}

impl Default for PersistentState {
    fn default() -> Self {
        Self {
            provider: "asimov-default-provider".into(),
        }
    }
}

/// Get the path to persistence file.
fn get_file_path() -> Result<PathBuf> {
    let current_dir = std::env::current_exe()?;
    Ok(current_dir.with_file_name("persistence.json"))
}

/// Read the persistent state from the file.
fn read() -> Result<PersistentState> {
    let path = get_file_path()?;
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

/// Write the persistent state to the file.
fn write(state: &PersistentState) -> Result<()> {
    let path = get_file_path()?;
    let file = std::fs::File::create(path)?;
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer(writer, state)?;
    Ok(())
}

/// Get the reference to persistent state.
pub(crate) fn get_ref() -> Arc<RwLock<PersistentState>> {
    STATE.clone()
}

/// Get the copy of current persistent state.
pub fn get() -> PersistentState {
    STATE.read().unwrap().clone()
}

/// Set the persistent state.
///
/// This method updates the persistent state
/// and writes it to the file _immediately_!
///
/// This method is thread-safe.
pub fn set<F>(x: F) -> Result<()>
where
    F: FnOnce(&mut PersistentState),
{
    let mut state = STATE.write().unwrap();
    x(&mut state);
    write(&state)
}
