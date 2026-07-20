// This is free and unencumbered software released into the public domain.

//! Module authoring support.

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use cargo_generate::{GenerateArgs, TemplatePath, Vcs, generate};
use std::{
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
};
use thiserror::Error;

pub mod cargo_toml;
#[cfg(feature = "lint")]
pub mod lint;
pub mod manifest_edit;
pub mod program;

pub const DEFAULT_TEMPLATE_GIT: &str = "https://github.com/asimov-modules/asimov-template-module";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TemplateSource {
    Git(String),
    Path(PathBuf),
}

impl Default for TemplateSource {
    fn default() -> Self {
        Self::Git(DEFAULT_TEMPLATE_GIT.into())
    }
}

#[derive(Clone, Debug)]
pub struct NewModuleOptions {
    pub target_dir: PathBuf,
    pub name: String,
    pub template: TemplateSource,
    pub branch: Option<String>,
    pub package_name: Option<String>,
    pub package_description: Option<String>,
    pub package_authors: Vec<String>,
    pub module_label: Option<String>,
    pub module_title: Option<String>,
    pub module_summary: Option<String>,
    pub program_name: Option<String>,
    pub repository_url: Option<String>,
    pub asimov_version: Option<String>,
    pub publish: bool,
    pub vcs: Option<String>,
    /// Whether to scaffold an initial program at all. Defaults to `true`;
    /// set to `false` (via [`Self::without_program`]) to create a bare
    /// module with no programs — add them later via [`program::add_program`].
    pub create_program: bool,
}

impl NewModuleOptions {
    pub fn new(target_dir: impl Into<PathBuf>, name: impl Into<String>) -> Self {
        Self {
            target_dir: target_dir.into(),
            name: name.into(),
            template: TemplateSource::default(),
            branch: None,
            package_name: None,
            package_description: None,
            package_authors: Vec::new(),
            module_label: None,
            module_title: None,
            module_summary: None,
            program_name: None,
            repository_url: None,
            asimov_version: Some(env!("CARGO_PKG_VERSION").into()),
            publish: false,
            vcs: Some("none".into()),
            create_program: true,
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

    /// Creates a bare module with no initial program.
    pub fn without_program(mut self) -> Self {
        self.create_program = false;
        self
    }
}

#[derive(Debug, Error)]
pub enum NewModuleError {
    #[error("module name must not be empty")]
    EmptyName,

    #[error("module name `{0}` is not supported; use lowercase letters, digits, and hyphens")]
    InvalidName(String),

    #[error("target directory has no parent: {0}")]
    MissingTargetParent(PathBuf),

    #[error("target directory has no final path component: {0}")]
    MissingTargetName(PathBuf),

    #[error("target directory already exists: {0}")]
    TargetExists(PathBuf),

    #[error("failed to create target parent directory `{0}`: {1}")]
    CreateTargetParent(PathBuf, #[source] io::Error),

    #[error("failed to remove unused program scaffold at `{0}`: {1}")]
    RemoveOrphanProgram(PathBuf, #[source] io::Error),

    #[error("failed to run `cargo generate`: {0}")]
    CargoGenerate(#[source] anyhow::Error),

    #[error(transparent)]
    InvalidProgramName(#[from] InvalidProgramName),
}

/// The program name doesn't match the `asimov-<module>-<kind>` shape.
///
/// The `<kind>` segment itself is never checked against any fixed list —
/// any lowercase, hyphenated word is accepted; only the overall shape
/// (matching the naming convention already enforced for module names) is
/// validated here.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
#[error(
    "program name `{0}` is not supported; use the shape `asimov-<module>-<kind>` (lowercase letters, digits, and hyphens)"
)]
pub struct InvalidProgramName(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreatedModule {
    pub module_name: String,
    pub crate_name: String,
    pub program_name: String,
    pub target_dir: PathBuf,
}

pub fn new_module(options: NewModuleOptions) -> Result<CreatedModule, NewModuleError> {
    tracing::info!(
        module = %options.name,
        target = ?options.target_dir,
        template = ?options.template,
        "generating new ASIMOV module"
    );

    validate_module_name(&options.name)?;
    if let Some(program_name) = &options.program_name {
        validate_program_name(program_name)?;
    }

    if options.target_dir.exists() {
        tracing::error!(target = ?options.target_dir, "target directory already exists");
        return Err(NewModuleError::TargetExists(options.target_dir));
    }

    let target_parent = options
        .target_dir
        .parent()
        .ok_or_else(|| NewModuleError::MissingTargetParent(options.target_dir.clone()))?;
    let target_name = target_dir_name(&options.target_dir)?;

    fs::create_dir_all(target_parent)
        .map_err(|err| NewModuleError::CreateTargetParent(target_parent.into(), err))?;

    let crate_name = options
        .package_name
        .clone()
        .unwrap_or_else(|| format!("asimov-{}-module", options.name));
    let module_label = options
        .module_label
        .clone()
        .unwrap_or_else(|| title_case_slug(&options.name));
    let module_title = options
        .module_title
        .clone()
        .unwrap_or_else(|| format!("ASIMOV {module_label} Module"));
    let module_summary = options
        .module_summary
        .clone()
        .unwrap_or_else(|| "ASIMOV module.".into());
    let package_description = options
        .package_description
        .clone()
        .unwrap_or_else(|| module_summary.clone());
    let package_authors = if options.package_authors.is_empty() {
        "ASIMOV Community".into()
    } else {
        options.package_authors.join(", ")
    };
    let program_name = options
        .program_name
        .clone()
        .unwrap_or_else(|| format!("asimov-{}-emitter", options.name));
    let program_kind = program_kind_of(&program_name).to_string();
    let repository_url = options
        .repository_url
        .clone()
        .unwrap_or_else(|| format!("https://github.com/asimov-modules/{crate_name}"));
    let asimov_version = options
        .asimov_version
        .clone()
        .unwrap_or_else(|| env!("CARGO_PKG_VERSION").into());

    let mut args = GenerateArgs {
        template_path: template_path(&options),
        name: Some(target_name.to_string_lossy().into_owned()),
        force: true,
        silent: true,
        destination: Some(target_parent.into()),
        vcs: options.vcs.as_deref().map(parse_vcs).transpose()?,
        no_workspace: true,
        ..GenerateArgs::default()
    };

    args.define = [
        ("package_name", crate_name.as_str()),
        ("package_description", package_description.as_str()),
        ("package_authors", package_authors.as_str()),
        ("module_name", options.name.as_str()),
        ("module_label", module_label.as_str()),
        ("module_title", module_title.as_str()),
        ("module_summary", module_summary.as_str()),
        ("program_name", program_name.as_str()),
        ("program_kind", program_kind.as_str()),
        ("repository_url", repository_url.as_str()),
        ("asimov_version", asimov_version.as_str()),
        ("publish", if options.publish { "true" } else { "false" }),
        (
            "create_program",
            if options.create_program {
                "true"
            } else {
                "false"
            },
        ),
    ]
    .into_iter()
    .map(|(key, value)| format!("{key}={value}"))
    .collect();

    generate(args).map_err(|err| {
        tracing::error!(error = %err, "cargo-generate failed");
        NewModuleError::CargoGenerate(err)
    })?;

    if !options.create_program {
        // The template still physically generates the program's source file
        // even when unreferenced (a cargo-generate limitation with templated
        // ignore paths) — clean up that orphan directory ourselves.
        let orphan_dir = options.target_dir.join("src").join(&program_kind);
        if orphan_dir.exists() {
            fs::remove_dir_all(&orphan_dir)
                .map_err(|err| NewModuleError::RemoveOrphanProgram(orphan_dir, err))?;
        }
    }

    tracing::info!(
        module = %options.name,
        crate_name = %crate_name,
        target = ?options.target_dir,
        "module generated"
    );

    Ok(CreatedModule {
        module_name: options.name,
        crate_name,
        program_name,
        target_dir: options.target_dir,
    })
}

fn template_path(options: &NewModuleOptions) -> TemplatePath {
    let mut template_path = TemplatePath::default();
    match &options.template {
        TemplateSource::Git(git) => {
            template_path.git = Some(git.clone());
            template_path.branch.clone_from(&options.branch);
        },
        TemplateSource::Path(path) => {
            template_path.path = Some(path.to_string_lossy().into_owned());
        },
    }
    template_path
}

fn parse_vcs(vcs: &str) -> Result<Vcs, NewModuleError> {
    vcs.parse().map_err(NewModuleError::CargoGenerate)
}

fn validate_module_name(name: &str) -> Result<(), NewModuleError> {
    if name.is_empty() {
        return Err(NewModuleError::EmptyName);
    }

    if !name.as_bytes()[0].is_ascii_alphanumeric() {
        return Err(NewModuleError::InvalidName(name.into()));
    }

    if name.ends_with('-') {
        return Err(NewModuleError::InvalidName(name.into()));
    }

    let valid = name
        .bytes()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-');
    if valid {
        Ok(())
    } else {
        Err(NewModuleError::InvalidName(name.into()))
    }
}

fn validate_program_name(name: &str) -> Result<(), InvalidProgramName> {
    let Some(rest) = name.strip_prefix("asimov-") else {
        return Err(InvalidProgramName(name.into()));
    };

    if rest.is_empty() || rest.starts_with('-') || rest.ends_with('-') || !rest.contains('-') {
        return Err(InvalidProgramName(name.into()));
    }

    let valid = rest
        .bytes()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-');
    if valid {
        Ok(())
    } else {
        Err(InvalidProgramName(name.into()))
    }
}

/// Derives a program's "kind" from its name — the last hyphen segment.
/// Never validated against any fixed list; any word is accepted.
pub(crate) fn program_kind_of(program_name: &str) -> &str {
    program_name.rsplit('-').next().unwrap_or(program_name)
}

fn target_dir_name(target_dir: &Path) -> Result<OsString, NewModuleError> {
    target_dir
        .file_name()
        .map(OsString::from)
        .ok_or_else(|| NewModuleError::MissingTargetName(target_dir.into()))
}

fn title_case_slug(slug: &str) -> String {
    slug.split('-')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => {
                    let mut word = first.to_uppercase().collect::<String>();
                    word.push_str(chars.as_str());
                    word
                },
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn rejects_invalid_module_names() {
        assert!(matches!(
            new_module(NewModuleOptions::new("target", "")),
            Err(NewModuleError::EmptyName)
        ));
        assert!(matches!(
            new_module(NewModuleOptions::new("target", "Not Valid")),
            Err(NewModuleError::InvalidName(_))
        ));
    }

    #[test]
    fn rejects_invalid_program_name() {
        let mut options = NewModuleOptions::new("target", "widget");
        options.program_name = Some("Not-Valid".into());
        assert!(matches!(
            new_module(options),
            Err(NewModuleError::InvalidProgramName(_))
        ));
    }

    #[test]
    fn validates_program_name_shape() {
        assert!(validate_program_name("asimov-widget-emitter").is_ok());
        assert!(validate_program_name("asimov-widget-multi-word-kind").is_ok());
        // The kind itself is never restricted to a known list:
        assert!(validate_program_name("asimov-widget-whatever-custom-kind").is_ok());

        assert!(validate_program_name("").is_err());
        assert!(validate_program_name("widget-emitter").is_err());
        assert!(validate_program_name("asimov-widget").is_err());
        assert!(validate_program_name("asimov-Widget-Emitter").is_err());
        assert!(validate_program_name("asimov-widget-emitter-").is_err());
        assert!(validate_program_name("asimov-widgetemitter").is_err());
    }

    #[test]
    fn generates_module_from_local_template() {
        let template_dir = tempdir().unwrap();
        fs::write(
            template_dir.path().join("Cargo.toml"),
            "[package]\nname = \"{{package_name}}\"\ndescription = \"{{package_description}}\"\n",
        )
        .unwrap();
        fs::create_dir(template_dir.path().join("src")).unwrap();
        fs::write(
            template_dir.path().join("src/lib.rs"),
            "// {{module_title}}\npub const PROGRAM: &str = \"{{program_name}}\";\n",
        )
        .unwrap();

        let workspace = tempdir().unwrap();
        let target_dir = workspace.path().join("widget-module");

        let options =
            NewModuleOptions::new(&target_dir, "widget").template_path(template_dir.path());
        let created = new_module(options).unwrap();

        assert_eq!(created.crate_name, "asimov-widget-module");
        assert_eq!(created.program_name, "asimov-widget-emitter");
        assert_eq!(created.target_dir, target_dir);

        let cargo_toml = fs::read_to_string(target_dir.join("Cargo.toml")).unwrap();
        assert!(cargo_toml.contains("name = \"asimov-widget-module\""));
        assert!(cargo_toml.contains("description = \"ASIMOV module.\""));

        let lib_rs = fs::read_to_string(target_dir.join("src/lib.rs")).unwrap();
        assert!(lib_rs.contains("ASIMOV Widget Module"));
        assert!(lib_rs.contains("asimov-widget-emitter"));
    }

    #[test]
    fn refuses_to_overwrite_existing_target() {
        let workspace = tempdir().unwrap();
        let target_dir = workspace.path().join("existing");
        fs::create_dir(&target_dir).unwrap();

        let options = NewModuleOptions::new(&target_dir, "widget");
        assert!(matches!(
            new_module(options),
            Err(NewModuleError::TargetExists(_))
        ));
    }

    #[test]
    fn passes_program_kind_for_a_custom_program_name() {
        let template_dir = tempdir().unwrap();
        fs::write(
            template_dir.path().join("Cargo.toml"),
            "[package]\nname = \"{{package_name}}\"\n",
        )
        .unwrap();
        fs::create_dir(template_dir.path().join("src")).unwrap();
        fs::write(
            template_dir.path().join("src/lib.rs"),
            "// kind: {{program_kind}}\n",
        )
        .unwrap();

        let workspace = tempdir().unwrap();
        let target_dir = workspace.path().join("widget-module");

        let mut options =
            NewModuleOptions::new(&target_dir, "widget").template_path(template_dir.path());
        options.program_name = Some("asimov-widget-fetcher".into());
        new_module(options).unwrap();

        let lib_rs = fs::read_to_string(target_dir.join("src/lib.rs")).unwrap();
        assert!(lib_rs.contains("kind: fetcher"));
    }

    #[test]
    fn creates_module_without_a_program() {
        let template_dir = tempdir().unwrap();
        fs::write(
            template_dir.path().join("Cargo.toml"),
            concat!(
                "[package]\n",
                "name = \"{{package_name}}\"\n",
                "\n",
                "{%- if create_program == \"true\" %}\n",
                "[[bin]]\n",
                "name = \"{{program_name}}\"\n",
                "path = \"src/{{program_kind}}/main.rs\"\n",
                "{%- endif %}\n",
            ),
        )
        .unwrap();
        fs::create_dir(template_dir.path().join("src")).unwrap();
        fs::write(template_dir.path().join("src/lib.rs"), "// lib\n").unwrap();
        // Simulate the real template's known cargo-generate limitation: the
        // program's source file physically gets generated regardless of
        // `create_program`, since a templated `ignore` path doesn't reliably
        // exclude it (see module::new_module's orphan-cleanup step).
        fs::create_dir(template_dir.path().join("src/emitter")).unwrap();
        fs::write(
            template_dir.path().join("src/emitter/main.rs"),
            "fn main() {}",
        )
        .unwrap();

        let workspace = tempdir().unwrap();
        let target_dir = workspace.path().join("bare-module");

        let options = NewModuleOptions::new(&target_dir, "bare")
            .template_path(template_dir.path())
            .without_program();
        new_module(options).unwrap();

        assert!(!target_dir.join("src/emitter").exists());
        let cargo_toml = fs::read_to_string(target_dir.join("Cargo.toml")).unwrap();
        assert!(!cargo_toml.contains("[[bin]]"));
    }
}
