// This is free and unencumbered software released into the public domain.

use serde::Deserialize;
use std::{string::String, vec::Vec};

#[derive(Debug, Deserialize)]
pub struct GitHubRelease {
    pub name: String,
    pub assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
}
