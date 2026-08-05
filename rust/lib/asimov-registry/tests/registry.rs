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
    module_dir: PathBuf,
    enabled_path: PathBuf,
    is_relative: bool,
) {
    registry.create_file_tree().await.unwrap();
    assert_eq!(registry.installed_modules().await.unwrap().len(), 0);

    tokio::fs::create_dir_all(&module_dir).await.unwrap();
    tokio::fs::write(module_dir.join("manifest.yaml"), SAMPLE_MANIFEST)
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
        std::fs::canonicalize(&module_dir).unwrap()
    );

    assert_eq!(registry.read_readme("sample").await.unwrap(), None);
    registry.add_readme("sample", "# Sample\n").await.unwrap();
    assert_eq!(
        registry.read_readme("sample").await.unwrap().as_deref(),
        Some("# Sample\n")
    );
    assert!(module_dir.join("doc/README.md").is_file());

    registry.remove_module("sample").await.unwrap();
    assert!(!module_dir.exists());
    assert_eq!(registry.installed_modules().await.unwrap().len(), 0);
}

#[tokio::test]
pub async fn test_default_registry() {
    let base_dir = tempdir().unwrap();
    let registry = Registry::new(base_dir.path(), Default::default());

    let module_dir = base_dir.path().join("modules/installed/sample");
    let enabled_path = base_dir.path().join("modules/enabled/sample");

    test_registry(registry, module_dir, enabled_path, true).await;
}

#[tokio::test]
pub async fn test_custom_registry() {
    let base_dir = tempdir().unwrap();
    let module_dir = base_dir.path().join("a/b/c/sample");
    let enabled_path = base_dir.path().join("b/sample");
    let libexec_path = base_dir.path().join("c");

    let registry = Registry::with_dirs(
        module_dir.parent().unwrap(),
        enabled_path.parent().unwrap(),
        libexec_path,
        Default::default(),
    );
    test_registry(registry, module_dir, enabled_path, false).await;
}

#[tokio::test]
pub async fn test_add_manifest() {
    let base_dir = tempdir().unwrap();
    let registry = Registry::new(base_dir.path(), Default::default());
    registry.create_file_tree().await.unwrap();

    let manifest = InstalledModuleManifest {
        version: Some("1.2.3".into()),
        manifest: serde_yaml_ng::from_str(SAMPLE_MANIFEST).unwrap(),
    };
    registry.add_manifest(manifest).await.unwrap();

    let module_dir = base_dir.path().join("modules/installed/ipfs");
    assert!(module_dir.join("manifest.json").is_file());

    let module = registry.read_manifest("ipfs").await.unwrap();
    assert_eq!(module.manifest.name, "ipfs");
    assert_eq!(module.version.as_deref(), Some("1.2.3"));
}

#[tokio::test]
pub async fn test_add_binary() {
    let base_dir = tempdir().unwrap();
    let registry = Registry::new(base_dir.path(), Default::default());
    registry.create_file_tree().await.unwrap();

    let source_path = base_dir.path().join("asimov-ipfs-fetcher");
    tokio::fs::write(&source_path, "#!/bin/sh\n").await.unwrap();

    let libexec_path = base_dir.path().join("libexec/asimov-ipfs-fetcher");

    tokio::fs::write(&libexec_path, "stale").await.unwrap();

    registry
        .add_binary("ipfs", "asimov-ipfs-fetcher", &source_path)
        .await
        .unwrap();

    let binary_path = base_dir
        .path()
        .join("modules/installed/ipfs/bin/asimov-ipfs-fetcher");
    assert!(binary_path.is_file());

    assert!(
        std::fs::symlink_metadata(&libexec_path)
            .unwrap()
            .is_symlink()
    );
    assert_eq!(
        std::fs::read_link(&libexec_path).unwrap(),
        PathBuf::from("../modules/installed/ipfs/bin/asimov-ipfs-fetcher")
    );
    assert_eq!(
        std::fs::canonicalize(&libexec_path).unwrap(),
        std::fs::canonicalize(&binary_path).unwrap()
    );

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(&binary_path)
            .unwrap()
            .permissions()
            .mode();
        assert_eq!(mode & 0o777, 0o755);
    }

    registry.remove_binary("asimov-ipfs-fetcher").await.unwrap();
    assert!(!libexec_path.exists());
    assert!(binary_path.is_file());
}

#[tokio::test]
pub async fn test_migrate_legacy_layouts() {
    for legacy_dir in ["modules", "modules/installed"] {
        let base_dir = tempdir().unwrap();
        let registry = Registry::new(base_dir.path(), Default::default());
        registry.create_file_tree().await.unwrap();

        let legacy_path = base_dir.path().join(legacy_dir).join("sample.yaml");
        tokio::fs::write(&legacy_path, SAMPLE_MANIFEST)
            .await
            .unwrap();

        assert!(registry.is_module_installed("sample").await.unwrap());
        registry.enable_module("sample").await.unwrap();

        let module_dir = base_dir.path().join("modules/installed/sample");
        assert!(!legacy_path.exists());
        assert!(module_dir.join("manifest.yaml").is_file());
        assert_eq!(registry.installed_modules().await.unwrap().len(), 1);
        assert_eq!(registry.enabled_modules().await.unwrap().len(), 1);

        let enabled_path = base_dir.path().join("modules/enabled/sample");
        let link_path = std::fs::read_link(&enabled_path).unwrap();
        assert_eq!(
            std::fs::canonicalize(enabled_path.parent().unwrap().join(link_path)).unwrap(),
            std::fs::canonicalize(&module_dir).unwrap()
        );
    }
}

#[tokio::test]
pub async fn test_migrate_legacy_version() {
    let base_dir = tempdir().unwrap();
    let registry = Registry::new(base_dir.path(), Default::default());
    registry.create_file_tree().await.unwrap();

    let legacy_path = base_dir.path().join("modules/installed/sample.json");
    let manifest = InstalledModuleManifest {
        version: Some("0.1.7".into()),
        manifest: serde_yaml_ng::from_str(SAMPLE_MANIFEST).unwrap(),
    };
    tokio::fs::write(&legacy_path, serde_json::to_vec(&manifest).unwrap())
        .await
        .unwrap();

    assert_eq!(
        registry.module_version("sample").await.unwrap().as_deref(),
        Some("0.1.7")
    );
    assert!(
        base_dir
            .path()
            .join("modules/installed/sample/manifest.json")
            .is_file()
    );
}
