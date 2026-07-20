// This is free and unencumbered software released into the public domain.

//! A narrow text-based edit for `.asimov/module.yaml`, deliberately not a
//! full YAML parse+reserialize: every real manifest opens with a `# See:
//! ...` comment and represents empty `handles:` fields as bare keys, and a
//! serde round-trip would silently drop the comment and reformat those
//! fields — an unacceptable blast radius for what should be a one-line
//! insertion.

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use std::{fs, io, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ManifestEditError {
    #[error("`provides.programs` block not found in expected shape in {0}")]
    UnexpectedShape(std::path::PathBuf),

    #[error(transparent)]
    Io(#[from] io::Error),
}

/// Appends `program_name` as a new item under `provides.programs:` in an
/// `.asimov/module.yaml` file, leaving every other line byte-identical.
pub fn append_provides_program(
    manifest_path: &Path,
    program_name: &str,
) -> Result<(), ManifestEditError> {
    let contents = fs::read_to_string(manifest_path)?;
    let updated = insert_program_line(&contents, program_name)
        .ok_or_else(|| ManifestEditError::UnexpectedShape(manifest_path.to_path_buf()))?;
    fs::write(manifest_path, updated)?;
    Ok(())
}

fn insert_program_line(contents: &str, program_name: &str) -> Option<String> {
    let lines: Vec<&str> = contents.lines().collect();

    let provides_idx = lines.iter().position(|line| *line == "provides:")?;
    let programs_idx = lines
        .iter()
        .enumerate()
        .skip(provides_idx + 1)
        .take_while(|(_, line)| line.starts_with(' ') || line.is_empty())
        .find(|(_, line)| line.trim_start() == "programs:")
        .map(|(i, _)| i)?;

    let indent = " ".repeat(4);
    let mut insert_at = programs_idx + 1;
    while lines
        .get(insert_at)
        .is_some_and(|line| line.trim_start().starts_with("- "))
    {
        insert_at += 1;
    }

    let mut result: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    result.insert(insert_at, format!("{indent}- {program_name}"));

    let mut joined = result.join("\n");
    if contents.ends_with('\n') {
        joined.push('\n');
    }
    Some(joined)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    const MANIFEST: &str = r#"# See: https://asimov-specs.github.io/module-manifest/
---
name: widget
label: Widget
title: ASIMOV Widget Module
summary: ASIMOV module.
links:
  - https://github.com/asimov-modules/asimov-widget-module

provides:
  programs:
    - asimov-widget-emitter

handles:
  url_protocols:
  url_prefixes:
  url_patterns:
  file_extensions:
  content_types:
"#;

    #[test]
    fn appends_after_existing_program() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("module.yaml");
        fs::write(&path, MANIFEST).unwrap();

        append_provides_program(&path, "asimov-widget-fetcher").unwrap();

        let updated = fs::read_to_string(&path).unwrap();
        let expected = MANIFEST.replace(
            "    - asimov-widget-emitter\n",
            "    - asimov-widget-emitter\n    - asimov-widget-fetcher\n",
        );
        assert_eq!(updated, expected);
    }

    #[test]
    fn appends_to_empty_program_list() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("module.yaml");
        let manifest = MANIFEST.replace("    - asimov-widget-emitter\n", "");
        fs::write(&path, &manifest).unwrap();

        append_provides_program(&path, "asimov-widget-emitter").unwrap();

        let updated = fs::read_to_string(&path).unwrap();
        assert_eq!(updated, MANIFEST);
    }

    #[test]
    fn preserves_everything_else_byte_for_byte() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("module.yaml");
        fs::write(&path, MANIFEST).unwrap();

        append_provides_program(&path, "asimov-widget-fetcher").unwrap();

        let updated = fs::read_to_string(&path).unwrap();
        for line in MANIFEST.lines() {
            assert!(updated.contains(line), "missing original line: {line:?}");
        }
        assert!(updated.starts_with("# See: https://asimov-specs.github.io/module-manifest/\n"));
    }

    #[test]
    fn errors_on_unexpected_shape() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("module.yaml");
        fs::write(&path, "name: widget\n").unwrap();

        let err = append_provides_program(&path, "asimov-widget-emitter").unwrap_err();
        assert!(matches!(err, ManifestEditError::UnexpectedShape(_)));
    }
}
