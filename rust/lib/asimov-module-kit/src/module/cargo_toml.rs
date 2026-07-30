// This is free and unencumbered software released into the public domain.

//! Targeted `Cargo.toml` edits that preserve existing formatting and comments.

use alloc::string::String;
use thiserror::Error;
use toml_edit::{Array, DocumentMut, Item, Table, value};

#[derive(Debug, Error)]
pub enum CargoTomlError {
    #[error("failed to parse Cargo.toml: {0}")]
    Parse(#[from] toml_edit::TomlError),

    #[error("`[bin]` in Cargo.toml is not an array of tables")]
    BinNotArrayOfTables,
}

/// Inserts a new `[[bin]]` entry, preserving all other formatting/comments.
pub fn insert_bin(
    doc: &mut DocumentMut,
    name: &str,
    path: &str,
    required_features: &[String],
) -> Result<(), CargoTomlError> {
    let mut bin = Table::new();
    bin.insert("name", value(name));
    bin.insert("path", value(path));
    if !required_features.is_empty() {
        let features: Array = required_features.iter().map(String::as_str).collect();
        bin.insert("required-features", value(features));
    }

    doc.entry("bin")
        .or_insert(Item::ArrayOfTables(Default::default()))
        .as_array_of_tables_mut()
        .ok_or(CargoTomlError::BinNotArrayOfTables)?
        .push(bin);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn inserts_into_existing_bin_array() {
        let toml = r#"# comment kept as-is
[package]
name = "asimov-widget-module"

[[bin]]
name = "asimov-widget-emitter"
path = "src/emitter/main.rs"
required-features = ["cli"]
"#;
        let mut doc: DocumentMut = toml.parse().unwrap();
        insert_bin(
            &mut doc,
            "asimov-widget-fetcher",
            "src/fetcher/main.rs",
            &[String::from("cli")],
        )
        .unwrap();

        let rendered = doc.to_string();
        assert!(rendered.contains("# comment kept as-is"));
        assert!(rendered.contains("name = \"asimov-widget-emitter\""));
        assert!(rendered.contains("name = \"asimov-widget-fetcher\""));
        assert!(rendered.contains("path = \"src/fetcher/main.rs\""));

        let bins = doc["bin"].as_array_of_tables().unwrap();
        assert_eq!(bins.len(), 2);
    }

    #[test]
    fn creates_bin_array_when_absent() {
        let toml = "[package]\nname = \"asimov-widget-module\"\n";
        let mut doc: DocumentMut = toml.parse().unwrap();
        insert_bin(
            &mut doc,
            "asimov-widget-emitter",
            "src/emitter/main.rs",
            &[],
        )
        .unwrap();

        let bins = doc["bin"].as_array_of_tables().unwrap();
        assert_eq!(bins.len(), 1);
        assert_eq!(
            bins.get(0).unwrap()["name"].as_str(),
            Some("asimov-widget-emitter")
        );
        assert!(bins.get(0).unwrap().get("required-features").is_none());
    }

    #[test]
    fn omits_required_features_when_empty() {
        let mut doc: DocumentMut = "[package]\nname = \"x\"\n".parse().unwrap();
        insert_bin(&mut doc, "x-emitter", "src/emitter/main.rs", &[]).unwrap();
        let bins = doc["bin"].as_array_of_tables().unwrap();
        assert!(bins.get(0).unwrap().get("required-features").is_none());
    }

    #[test]
    fn includes_required_features_when_given() {
        let mut doc: DocumentMut = "[package]\nname = \"x\"\n".parse().unwrap();
        insert_bin(
            &mut doc,
            "x-emitter",
            "src/emitter/main.rs",
            &[String::from("cli"), String::from("std")],
        )
        .unwrap();
        let bins = doc["bin"].as_array_of_tables().unwrap();
        let features = bins.get(0).unwrap()["required-features"]
            .as_array()
            .unwrap();
        assert_eq!(features.len(), 2);
    }
}
