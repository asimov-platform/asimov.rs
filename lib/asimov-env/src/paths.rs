// This is free and unencumbered software released into the public domain.

use clientele::envs::var;
use std::path::PathBuf;

pub fn asimov_root() -> PathBuf {
    var("ASIMOV_ROOT")
        .map(PathBuf::from)
        .or_else(|| var("HOME").map(|p| PathBuf::from(p).join(".asimov")))
        .expect("ASIMOV_ROOT or HOME environment variable is set")
}

pub fn python_env() -> PathBuf {
    asimov_root().join("envs/python")
}

pub fn ruby_env() -> PathBuf {
    asimov_root().join("envs/ruby")
}
