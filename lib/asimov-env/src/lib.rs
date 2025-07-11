// This is free and unencumbered software released into the public domain.

pub mod env;
pub mod envs;

#[cfg(feature = "std")]
pub mod paths;

#[cfg(feature = "std")]
pub mod vars;

#[cfg(feature = "std")]
use cap_std::fs_utf8::Dir;

pub const ASIMOV_HOME: &str = "ASIMOV_HOME";

#[cfg(feature = "std")]
const CONFIG_PATH: &str = ".config/asimov";

/// Returns the home directory of the current user.
#[cfg(feature = "std")]
pub fn home_dir() -> std::io::Result<Dir> {
    use cap_directories::UserDirs;

    let ambient_authority = cap_std::ambient_authority();

    let user_dirs = UserDirs::new().unwrap();
    user_dirs.home_dir(ambient_authority).map(Dir::from_cap_std)
}

/// Returns the `ASIMOV_HOME` configuration directory.
#[cfg(feature = "std")]
pub fn config_dir() -> std::io::Result<Dir> {
    use std::{
        env::VarError,
        io::{Error, ErrorKind},
    };

    let ambient_authority = cap_std::ambient_authority();

    match std::env::var(ASIMOV_HOME) {
        Err(VarError::NotUnicode(_)) => Err(Error::new(
            ErrorKind::InvalidData,
            "ASIMOV_HOME is not valid Unicode",
        )),
        Err(VarError::NotPresent) => home_dir()
            .and_then(|home_dir| {
                home_dir.create_dir_all(CONFIG_PATH)?;
                Ok(home_dir)
            })
            .and_then(|home_dir| home_dir.open_dir(CONFIG_PATH)),
        Ok(path) if path.trim().is_empty() => {
            Err(Error::new(ErrorKind::InvalidData, "ASIMOV_HOME is empty"))
        },
        Ok(path) => Dir::create_ambient_dir_all(&path, ambient_authority)
            .and_then(|_| Dir::open_ambient_dir(&path, ambient_authority)),
    }
}
