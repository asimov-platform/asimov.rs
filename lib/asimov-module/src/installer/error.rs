// This is free and unencumbered software released into the public domain.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadError {}

#[derive(Debug, Error)]
pub enum FetchError {}

#[derive(Debug, Error)]
pub enum InstallError {}

#[derive(Debug, Error)]
pub enum UninstallError {}

#[derive(Debug, Error)]
pub enum EnableError {}

#[derive(Debug, Error)]
pub enum DisableError {}
