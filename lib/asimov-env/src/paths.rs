// This is free and unencumbered software released into the public domain.

use super::envs;
use std::path::PathBuf;

pub fn asimov_root() -> PathBuf {
    envs::asimov_root()
        .or_else(|| getenv::home().map(|p| PathBuf::from(p).join(".asimov")))
        .expect("ASIMOV_ROOT or HOME environment variable is set")
}

pub fn python_env() -> PathBuf {
    asimov_root().join("envs/python")
}

pub fn ruby_env() -> PathBuf {
    asimov_root().join("envs/ruby")
}
