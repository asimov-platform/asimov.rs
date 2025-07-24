// This is free and unencumbered software released into the public domain.

use std::string::String;

#[derive(Debug)]
pub struct PlatformInfo {
    pub os: String,
    pub arch: String,
    pub libc: Option<String>,
}

pub fn detect_platform() -> PlatformInfo {
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    let os = "unknown";
    #[cfg(target_os = "macos")]
    let os = "macos";
    #[cfg(target_os = "linux")]
    let os = "linux";
    #[cfg(target_os = "windows")]
    let os = "windows";

    #[cfg(not(any(target_arch = "aarch64", target_arch = "arm", target_arch = "x86_64")))]
    let arch = "unknown";
    #[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
    let arch = "arm";
    #[cfg(all(target_arch = "x86_64", not(target_os = "windows")))]
    let arch = "x86";
    #[cfg(all(target_arch = "x86_64", target_os = "windows"))]
    let arch = "x64";

    #[cfg(not(any(target_env = "musl", target_env = "gnu")))]
    let libc = None;
    #[cfg(target_env = "musl")]
    let libc = Some("musl".to_string());
    #[cfg(target_env = "gnu")]
    let libc = Some("gnu".to_string());

    PlatformInfo {
        os: os.into(),
        arch: arch.into(),
        libc,
    }
}
