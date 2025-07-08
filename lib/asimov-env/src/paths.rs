// This is free and unencumbered software released into the public domain.

use super::vars;
use std::path::PathBuf;

pub fn asimov_root() -> PathBuf {
    if let Some(asimov_root) = vars::asimov_root() {
        return asimov_root;
    }

    #[cfg(unix)]
    return getenv::home()
        .map(|p| PathBuf::from(p).join(".asimov"))
        .expect("ASIMOV_ROOT or HOME environment variables must be set");

    #[cfg(windows)]
    return getenv::appdata()
        .map(|p| PathBuf::from(p).join("ASIMOV"))
        .expect("ASIMOV_ROOT or APPDATA environment variables must be set");
}

pub fn python_env() -> PathBuf {
    asimov_root().join("envs/python")
}

pub fn ruby_env() -> PathBuf {
    asimov_root().join("envs/ruby")
}
