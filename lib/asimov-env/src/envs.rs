// This is free and unencumbered software released into the public domain.

use clientele::envs::var;
use std::path::PathBuf;

pub fn asimov_root() -> Option<PathBuf> {
    var("ASIMOV_ROOT").map(PathBuf::from)
}
