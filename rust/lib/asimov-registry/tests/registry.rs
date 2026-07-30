use std::path::PathBuf;

use asimov_module::InstalledModuleManifest;
use asimov_registry::Registry;
use tempfile::tempdir;

const SAMPLE_MANIFEST: &str = r#"# See: https://asimov-specs.github.io/module-manifest/
---
name: ipfs
label: IPFS
title: ASIMOV IPFS Module
summary: IPFS protocol support.
links:
  - https://github.com/asimov-modules/asimov-ipfs-module
  - https://crates.io/crates/asimov-ipfs-module

provides:
  programs:
    - asimov-ipfs-fetcher

handles:
  url_protocols:
    - ipfs
  url_prefixes:
  url_patterns:
  file_extensions:
  content_types:
"#;

pub fn compare_manifest(a: &InstalledModuleManifest, b: &InstalledModuleManifest) {
    assert_eq!(a.manifest.name, b.manifest.name);
    assert_eq!(a.manifest.label, b.manifest.label);
    assert_eq!(a.manifest.summary, b.manifest.summary);
    assert_eq!(a.manifest.links, b.manifest.links);
    assert_eq!(a.manifest.provides.programs, b.manifest.provides.programs);
    assert_eq!(
        a.manifest.handles.url_protocols,
        b.manifest.handles.url_protocols
    );
    assert_eq!(
        a.manifest.handles.url_prefixes,
        b.manifest.handles.url_prefixes
    );
    assert_eq!(
        a.manifest.handles.url_patterns,
        b.manifest.handles.url_patterns
    );
    assert_eq!(
        a.manifest.handles.file_extensions,
        b.manifest.handles.file_extensions
    );
    assert_eq!(
        a.manifest.handles.content_types,
        b.manifest.handles.content_types
    );
}

pub async fn test_registry(
    registry: Registry,
    installed_path: PathBuf,
    enabled_path: PathBuf,
    is_relative: bool,
) {
    registry.create_file_tree().await.unwrap();
    assert_eq!(registry.installed_modules().await.unwrap().len(), 0);

    tokio::fs::write(&installed_path, SAMPLE_MANIFEST)
        .await
        .unwrap();

    let module = registry.read_manifest("sample").await.unwrap();
    assert_eq!(module.manifest.name, "ipfs");
    assert_eq!(module.manifest.label.as_deref(), Some("IPFS"));
    assert_eq!(
        module.manifest.summary.as_deref(),
        Some("IPFS protocol support.")
    );

    let installed_modules = registry.installed_modules().await.unwrap();
    assert_eq!(installed_modules.len(), 1);
    compare_manifest(&installed_modules[0], &module);

    assert_eq!(registry.enabled_modules().await.unwrap().len(), 0);
    registry.enable_module("sample").await.unwrap();

    let enabled_modules = registry.enabled_modules().await.unwrap();
    assert_eq!(enabled_modules.len(), 1);
    compare_manifest(&enabled_modules[0], &module);

    let metadata = std::fs::symlink_metadata(&enabled_path).unwrap();
    assert!(metadata.is_symlink());

    let link_path = std::fs::read_link(&enabled_path).unwrap();
    assert_eq!(link_path.starts_with("../"), is_relative);

    let absolute_path = std::fs::canonicalize(enabled_path.parent().unwrap().join(link_path));
    assert_eq!(
        absolute_path.unwrap(),
        std::fs::canonicalize(&installed_path).unwrap()
    );
}

#[tokio::test]
pub async fn test_default_registry() {
    let base_dir = tempdir().unwrap();
    let registry = Registry::new(base_dir.path(), Default::default());

    let installed_path = base_dir.path().join("modules/installed/sample.yaml");
    let enabled_path = base_dir.path().join("modules/enabled/sample");

    test_registry(registry, installed_path, enabled_path, true).await;
}

#[tokio::test]
pub async fn test_custom_registry() {
    let base_dir = tempdir().unwrap();
    let installed_path = base_dir.path().join("a/b/c/sample.yaml");
    let enabled_path = base_dir.path().join("b/sample");
    let libexec_path = base_dir.path().join("c");

    let registry = Registry::with_dirs(
        installed_path.parent().unwrap(),
        enabled_path.parent().unwrap(),
        libexec_path,
        Default::default(),
    );
    test_registry(registry, installed_path, enabled_path, false).await;
}
