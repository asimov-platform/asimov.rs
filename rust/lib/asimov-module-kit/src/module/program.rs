// This is free and unencumbered software released into the public domain.

//! Adding a program to an existing module.

use super::{InvalidProgramName, cargo_toml, manifest_edit, validate_program_name};
use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use std::{fs, io, path::PathBuf};
use thiserror::Error;
use toml_edit::DocumentMut;

#[derive(Clone, Debug)]
pub struct AddProgramOptions {
    pub module_dir: PathBuf,
    pub program_name: String,
    /// Defaults to a sibling `[[bin]]`'s `required-features`, else `["cli"]`.
    pub required_features: Vec<String>,
    /// A local directory containing the template's
    /// `.template/program.rs.liquid` (e.g. an `asimov-template-module`
    /// checkout). Never a git URL — callers that only have one (like
    /// [`super::new_module`]) are responsible for resolving it to a local
    /// checkout first.
    pub template_path: PathBuf,
}

impl AddProgramOptions {
    pub fn new(
        module_dir: impl Into<PathBuf>,
        program_name: impl Into<String>,
        template_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            module_dir: module_dir.into(),
            program_name: program_name.into(),
            required_features: Vec::new(),
            template_path: template_path.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddedProgram {
    pub program_name: String,
    /// Derived from `program_name` by stripping the module's own
    /// `asimov-<module>-` prefix when it's known (from the module's own
    /// `Cargo.toml`), falling back to the last hyphen segment otherwise.
    /// Never validated against any fixed list of kinds — any word is
    /// accepted.
    pub kind: String,
    pub source_path: PathBuf,
    /// `false` if `.asimov/module.yaml` was missing or not in the expected
    /// shape, in which case it was left untouched.
    pub manifest_updated: bool,
}

#[derive(Debug, Error)]
pub enum AddProgramError {
    #[error(transparent)]
    InvalidProgramName(#[from] InvalidProgramName),

    #[error("not a module (no Cargo.toml found): {0}")]
    NotAModule(PathBuf),

    #[error(
        "program `{program_name}` doesn't belong to this module; expected the shape `asimov-{module_name}-<kind>`"
    )]
    ProgramModuleMismatch {
        program_name: String,
        module_name: String,
    },

    #[error("program `{0}` already declared in Cargo.toml")]
    ProgramAlreadyExists(String),

    #[error("source path already exists: {0}")]
    SourcePathExists(PathBuf),

    #[error("failed to parse Cargo.toml: {0}")]
    CargoToml(#[source] toml_edit::TomlError),

    #[error(transparent)]
    CargoTomlEdit(#[from] cargo_toml::CargoTomlError),

    #[error(transparent)]
    ManifestEdit(#[from] manifest_edit::ManifestEditError),

    #[error("failed to render program source: {0}")]
    Liquid(#[source] liquid::Error),

    #[error(transparent)]
    Io(#[from] io::Error),
}

pub fn add_program(options: AddProgramOptions) -> Result<AddedProgram, AddProgramError> {
    validate_program_name(&options.program_name)?;

    let cargo_toml_path = options.module_dir.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        return Err(AddProgramError::NotAModule(options.module_dir.clone()));
    }

    let mut doc: DocumentMut = fs::read_to_string(&cargo_toml_path)?
        .parse()
        .map_err(AddProgramError::CargoToml)?;

    // Best-effort: if the package name follows the usual
    // `asimov-<module>-module` convention, cross-check that the program
    // being added actually belongs to this module and not some other one,
    // and use it below to derive the program's kind precisely (rather than
    // the naive last-hyphen-segment heuristic, which breaks for
    // multi-hyphen custom kinds).
    let module_name: Option<String> = doc
        .get("package")
        .and_then(|t| t.get("name"))
        .and_then(|v| v.as_str())
        .and_then(|name| name.strip_prefix("asimov-"))
        .and_then(|name| name.strip_suffix("-module"))
        .map(String::from);
    if let Some(module_name) = &module_name {
        let expected_prefix = format!("asimov-{module_name}-");
        if !options.program_name.starts_with(&expected_prefix) {
            return Err(AddProgramError::ProgramModuleMismatch {
                program_name: options.program_name,
                module_name: module_name.clone(),
            });
        }
    }

    let mut required_features = options.required_features.clone();
    if let Some(bins) = doc.get("bin").and_then(|item| item.as_array_of_tables()) {
        for bin in bins.iter() {
            if bin.get("name").and_then(|v| v.as_str()) == Some(options.program_name.as_str()) {
                return Err(AddProgramError::ProgramAlreadyExists(options.program_name));
            }
            if required_features.is_empty()
                && let Some(features) = bin.get("required-features").and_then(|v| v.as_array())
            {
                required_features = features
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
            }
        }
    }

    let kind = module_name
        .as_deref()
        .and_then(|module_name| {
            options
                .program_name
                .strip_prefix(&format!("asimov-{module_name}-"))
        })
        .unwrap_or_else(|| super::program_kind_of(&options.program_name))
        .to_string();
    let relative_path = format!("src/{kind}/main.rs");
    let source_path = options.module_dir.join(&relative_path);
    if source_path.exists() {
        return Err(AddProgramError::SourcePathExists(source_path));
    }

    if required_features.is_empty() {
        required_features = vec![String::from("cli")];
    }

    let contents = render_program_source(&options, &kind)?;
    fs::create_dir_all(
        source_path
            .parent()
            .expect("source_path always has a parent"),
    )?;
    fs::write(&source_path, contents)?;

    cargo_toml::insert_bin(
        &mut doc,
        &options.program_name,
        &relative_path,
        &required_features,
    )?;
    fs::write(&cargo_toml_path, doc.to_string())?;

    let manifest_path = options.module_dir.join(".asimov/module.yaml");
    let manifest_updated = if manifest_path.exists() {
        match manifest_edit::append_provides_program(&manifest_path, &options.program_name) {
            Ok(()) => true,
            Err(manifest_edit::ManifestEditError::UnexpectedShape(_)) => false,
            Err(err) => return Err(err.into()),
        }
    } else {
        false
    };

    Ok(AddedProgram {
        program_name: options.program_name,
        kind,
        source_path,
        manifest_updated,
    })
}

fn render_program_source(
    options: &AddProgramOptions,
    kind: &str,
) -> Result<String, AddProgramError> {
    let liquid_path = options.template_path.join(".template/program.rs.liquid");
    let text = fs::read_to_string(&liquid_path)?;
    let parser = liquid::ParserBuilder::with_stdlib()
        .build()
        .map_err(AddProgramError::Liquid)?;
    let template = parser.parse(&text).map_err(AddProgramError::Liquid)?;
    let globals = liquid::object!({
        "program_name": options.program_name.clone(),
        "program_kind": kind.to_string(),
    });
    template.render(&globals).map_err(AddProgramError::Liquid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn fixture_template(dir: &std::path::Path) {
        fs::create_dir_all(dir.join(".template")).unwrap();
        fs::write(
            dir.join(".template/program.rs.liquid"),
            "// {{ program_name }} ({{ program_kind }})\nfn main() { println!(\"{{ program_name }}\"); }\n",
        )
        .unwrap();
    }

    fn fixture_module(dir: &std::path::Path) {
        fs::create_dir_all(dir.join("src/emitter")).unwrap();
        fs::write(
            dir.join("Cargo.toml"),
            r#"
[package]
name = "asimov-widget-module"

[[bin]]
name = "asimov-widget-emitter"
path = "src/emitter/main.rs"
required-features = ["cli"]
"#,
        )
        .unwrap();
        fs::write(dir.join("src/emitter/main.rs"), "fn main() {}").unwrap();
        fs::create_dir_all(dir.join(".asimov")).unwrap();
        fs::write(
            dir.join(".asimov/module.yaml"),
            "provides:\n  programs:\n    - asimov-widget-emitter\n",
        )
        .unwrap();
    }

    #[test]
    fn adds_a_program_from_local_template() {
        let template_dir = tempdir().unwrap();
        fixture_template(template_dir.path());

        let module_dir = tempdir().unwrap();
        fixture_module(module_dir.path());

        let options = AddProgramOptions::new(
            module_dir.path(),
            "asimov-widget-fetcher",
            template_dir.path(),
        );
        let added = add_program(options).unwrap();

        assert_eq!(added.kind, "fetcher");
        assert!(added.manifest_updated);
        assert!(added.source_path.ends_with("src/fetcher/main.rs"));

        let source = fs::read_to_string(&added.source_path).unwrap();
        assert!(source.contains("asimov-widget-fetcher"));
        assert!(source.contains("(fetcher)"));

        let cargo_toml = fs::read_to_string(module_dir.path().join("Cargo.toml")).unwrap();
        assert!(cargo_toml.contains("name = \"asimov-widget-fetcher\""));
        assert!(cargo_toml.contains("path = \"src/fetcher/main.rs\""));
        // The existing bin's required-features convention was copied:
        assert!(cargo_toml.matches("required-features = [\"cli\"]").count() == 2);

        let manifest = fs::read_to_string(module_dir.path().join(".asimov/module.yaml")).unwrap();
        assert!(manifest.contains("asimov-widget-emitter"));
        assert!(manifest.contains("asimov-widget-fetcher"));
    }

    #[test]
    fn rejects_duplicate_program() {
        let template_dir = tempdir().unwrap();
        fixture_template(template_dir.path());
        let module_dir = tempdir().unwrap();
        fixture_module(module_dir.path());

        let options = AddProgramOptions::new(
            module_dir.path(),
            "asimov-widget-emitter",
            template_dir.path(),
        );
        assert!(matches!(
            add_program(options),
            Err(AddProgramError::ProgramAlreadyExists(_))
        ));
    }

    #[test]
    fn rejects_existing_source_path() {
        let template_dir = tempdir().unwrap();
        fixture_template(template_dir.path());
        let module_dir = tempdir().unwrap();
        fixture_module(module_dir.path());
        fs::create_dir_all(module_dir.path().join("src/fetcher")).unwrap();
        fs::write(
            module_dir.path().join("src/fetcher/main.rs"),
            "fn main() {}",
        )
        .unwrap();

        let options = AddProgramOptions::new(
            module_dir.path(),
            "asimov-widget-fetcher",
            template_dir.path(),
        );
        assert!(matches!(
            add_program(options),
            Err(AddProgramError::SourcePathExists(_))
        ));
    }

    #[test]
    fn rejects_program_name_not_matching_module() {
        let template_dir = tempdir().unwrap();
        fixture_template(template_dir.path());
        let module_dir = tempdir().unwrap();
        fixture_module(module_dir.path());

        let options = AddProgramOptions::new(
            module_dir.path(),
            "asimov-camera-fetcher",
            template_dir.path(),
        );
        assert!(matches!(
            add_program(options),
            Err(AddProgramError::ProgramModuleMismatch { .. })
        ));
    }

    #[test]
    fn rejects_not_a_module() {
        let template_dir = tempdir().unwrap();
        fixture_template(template_dir.path());
        let module_dir = tempdir().unwrap();

        let options = AddProgramOptions::new(
            module_dir.path(),
            "asimov-widget-fetcher",
            template_dir.path(),
        );
        assert!(matches!(
            add_program(options),
            Err(AddProgramError::NotAModule(_))
        ));
    }

    #[test]
    fn accepts_arbitrary_unlisted_kind() {
        let template_dir = tempdir().unwrap();
        fixture_template(template_dir.path());
        let module_dir = tempdir().unwrap();
        fixture_module(module_dir.path());

        let options = AddProgramOptions::new(
            module_dir.path(),
            "asimov-widget-whatever",
            template_dir.path(),
        );
        let added = add_program(options).unwrap();
        assert_eq!(added.kind, "whatever");
    }

    #[test]
    fn skips_manifest_update_when_missing() {
        let template_dir = tempdir().unwrap();
        fixture_template(template_dir.path());
        let module_dir = tempdir().unwrap();
        fixture_module(module_dir.path());
        fs::remove_dir_all(module_dir.path().join(".asimov")).unwrap();

        let options = AddProgramOptions::new(
            module_dir.path(),
            "asimov-widget-fetcher",
            template_dir.path(),
        );
        let added = add_program(options).unwrap();
        assert!(!added.manifest_updated);
        // The program source and Cargo.toml entry are still written:
        assert!(added.source_path.exists());
        let cargo_toml = fs::read_to_string(module_dir.path().join("Cargo.toml")).unwrap();
        assert!(cargo_toml.contains("asimov-widget-fetcher"));
    }

    #[test]
    fn propagates_manifest_io_errors_instead_of_swallowing_them() {
        let template_dir = tempdir().unwrap();
        fixture_template(template_dir.path());
        let module_dir = tempdir().unwrap();
        fixture_module(module_dir.path());

        // Replace the manifest file with a directory so reading it fails
        // with a real I/O error, not `UnexpectedShape`.
        fs::remove_file(module_dir.path().join(".asimov/module.yaml")).unwrap();
        fs::create_dir(module_dir.path().join(".asimov/module.yaml")).unwrap();

        let options = AddProgramOptions::new(
            module_dir.path(),
            "asimov-widget-fetcher",
            template_dir.path(),
        );
        assert!(matches!(
            add_program(options),
            Err(AddProgramError::ManifestEdit(
                manifest_edit::ManifestEditError::Io(_)
            ))
        ));
    }
}
