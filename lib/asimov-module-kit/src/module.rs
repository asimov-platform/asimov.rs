// This is free and unencumbered software released into the public domain.

//! Module authoring support.

use alloc::{format, string::String, vec::Vec};
use cargo_generate::{GenerateArgs, TemplatePath, Vcs, generate};
use std::{
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
};
use thiserror::Error;

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

    #[error("failed to run `cargo generate`: {0}")]
    CargoGenerate(#[source] anyhow::Error),
}

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
        ("repository_url", repository_url.as_str()),
        ("asimov_version", asimov_version.as_str()),
        ("publish", if options.publish { "true" } else { "false" }),
    ]
    .into_iter()
    .map(|(key, value)| format!("{key}={value}"))
    .collect();

    generate(args).map_err(|err| {
        tracing::error!(error = %err, "cargo-generate failed");
        NewModuleError::CargoGenerate(err)
    })?;

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
}
