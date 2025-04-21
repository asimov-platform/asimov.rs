// This is free and unencumbered software released into the public domain.

use std::path::PathBuf;

pub fn asimov_root() -> Option<PathBuf> {
    getenv::var("ASIMOV_ROOT").map(PathBuf::from)
}
