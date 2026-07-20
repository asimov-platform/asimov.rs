// This is free and unencumbered software released into the public domain.

//! Adding a program to an existing module.

use super::{InvalidProgramName, TemplateSource, cargo_toml, manifest_edit, validate_program_name};
use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use cargo_generate::{GenerateArgs, TemplatePath, Vcs, generate};
use std::{fs, io, path::PathBuf};
use thiserror::Error;
use toml_edit::DocumentMut;

#[derive(Clone, Debug)]
pub struct AddProgramOptions {
    pub module_dir: PathBuf,
    pub program_name: String,
    /// Defaults to a sibling `[[bin]]`'s `required-features`, else `["cli"]`.
    pub required_features: Vec<String>,
    pub template: TemplateSource,
    pub branch: Option<String>,
}

impl AddProgramOptions {
    pub fn new(module_dir: impl Into<PathBuf>, program_name: impl Into<String>) -> Self {
        Self {
            module_dir: module_dir.into(),
            program_name: program_name.into(),
            required_features: Vec::new(),
            template: TemplateSource::default(),
            branch: None,
        }
    }

    pub fn template_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.template = TemplateSource::Path(path.into());
        self
    }

    pub fn template_git(mut self, git: impl Into<String>) -> Self {
        self.template = TemplateSource::Git(git.into());
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddedProgram {
    pub program_name: String,
    /// Derived from `program_name`'s last hyphen segment. Never validated
    /// against any fixed list of kinds — any word is accepted.
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

    #[error("program `{0}` already declared in Cargo.toml")]
    ProgramAlreadyExists(String),

    #[error("source path already exists: {0}")]
    SourcePathExists(PathBuf),

    #[error("failed to parse Cargo.toml: {0}")]
    CargoToml(#[source] toml_edit::TomlError),

    #[error(transparent)]
    CargoTomlEdit(#[from] cargo_toml::CargoTomlError),

    #[error("failed to render program source: {0}")]
    Liquid(#[source] liquid::Error),

    #[error("failed to run `cargo generate`: {0}")]
    CargoGenerate(#[source] anyhow::Error),

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

    let kind = super::program_kind_of(&options.program_name).to_string();
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
    let manifest_updated = manifest_path.exists()
        && manifest_edit::append_provides_program(&manifest_path, &options.program_name).is_ok();

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
    match &options.template {
        TemplateSource::Path(path) => {
            let liquid_path = path.join("src/{{program_kind}}/main.rs.liquid");
            let text = fs::read_to_string(&liquid_path)?;
            let parser = liquid::ParserBuilder::with_stdlib()
                .build()
                .map_err(AddProgramError::Liquid)?;
            let template = parser.parse(&text).map_err(AddProgramError::Liquid)?;
            let globals = liquid::object!({ "program_name": options.program_name.clone() });
            template.render(&globals).map_err(AddProgramError::Liquid)
        },
        TemplateSource::Git(git) => {
            let scratch = tempfile::tempdir()?;
            let args = GenerateArgs {
                template_path: TemplatePath {
                    git: Some(git.clone()),
                    branch: options.branch.clone(),
                    ..TemplatePath::default()
                },
                name: Some("scratch-module".into()),
                force: true,
                silent: true,
                destination: Some(scratch.path().into()),
                vcs: Some(Vcs::None),
                no_workspace: true,
                define: [
                    ("package_name", "asimov-scratch-module"),
                    ("package_description", "scratch"),
                    ("package_authors", "ASIMOV Community"),
                    ("module_name", "scratch"),
                    ("module_label", "Scratch"),
                    ("module_title", "Scratch"),
                    ("module_summary", "scratch"),
                    ("program_name", options.program_name.as_str()),
                    ("program_kind", kind),
                    (
                        "repository_url",
                        "https://example.com/asimov-scratch-module",
                    ),
                    ("asimov_version", env!("CARGO_PKG_VERSION")),
                    ("publish", "false"),
                    ("create_program", "true"),
                ]
                .into_iter()
                .map(|(key, value)| format!("{key}={value}"))
                .collect(),
                ..GenerateArgs::default()
            };

            generate(args).map_err(AddProgramError::CargoGenerate)?;

            let rendered_path = scratch
                .path()
                .join("scratch-module/src")
                .join(kind)
                .join("main.rs");
            Ok(fs::read_to_string(rendered_path)?)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn fixture_template(dir: &std::path::Path) {
        fs::create_dir_all(dir.join("src/{{program_kind}}")).unwrap();
        fs::write(
            dir.join("src/{{program_kind}}/main.rs.liquid"),
            "// {{ program_name }}\nfn main() { println!(\"{{ program_name }}\"); }\n",
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

        let options = AddProgramOptions::new(module_dir.path(), "asimov-widget-fetcher")
            .template_path(template_dir.path());
        let added = add_program(options).unwrap();

        assert_eq!(added.kind, "fetcher");
        assert!(added.manifest_updated);
        assert!(added.source_path.ends_with("src/fetcher/main.rs"));

        let source = fs::read_to_string(&added.source_path).unwrap();
        assert!(source.contains("asimov-widget-fetcher"));

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

        let options = AddProgramOptions::new(module_dir.path(), "asimov-widget-emitter")
            .template_path(template_dir.path());
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

        let options = AddProgramOptions::new(module_dir.path(), "asimov-widget-fetcher")
            .template_path(template_dir.path());
        assert!(matches!(
            add_program(options),
            Err(AddProgramError::SourcePathExists(_))
        ));
    }

    #[test]
    fn rejects_not_a_module() {
        let template_dir = tempdir().unwrap();
        fixture_template(template_dir.path());
        let module_dir = tempdir().unwrap();

        let options = AddProgramOptions::new(module_dir.path(), "asimov-widget-fetcher")
            .template_path(template_dir.path());
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

        let options = AddProgramOptions::new(module_dir.path(), "asimov-widget-whatever")
            .template_path(template_dir.path());
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

        let options = AddProgramOptions::new(module_dir.path(), "asimov-widget-fetcher")
            .template_path(template_dir.path());
        let added = add_program(options).unwrap();
        assert!(!added.manifest_updated);
        // The program source and Cargo.toml entry are still written:
        assert!(added.source_path.exists());
        let cargo_toml = fs::read_to_string(module_dir.path().join("Cargo.toml")).unwrap();
        assert!(cargo_toml.contains("asimov-widget-fetcher"));
    }

    #[test]
    #[ignore = "network smoke test against the real default template; run manually"]
    fn smoke_test_default_git_template() {
        let module_dir = tempdir().unwrap();
        let new_module_options =
            crate::module::NewModuleOptions::new(module_dir.path().join("widget-module"), "widget")
                .without_program();
        let created = crate::module::new_module(new_module_options).unwrap();
        assert!(!created.target_dir.join("src/emitter").exists());

        let options = AddProgramOptions::new(&created.target_dir, "asimov-widget-fetcher")
            .template_git(crate::module::DEFAULT_TEMPLATE_GIT);
        let added = add_program(options).unwrap();
        assert_eq!(added.kind, "fetcher");
        assert!(added.source_path.exists());
    }
}
