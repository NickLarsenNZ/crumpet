use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use gix::{url::Scheme, Url};
use semver::Version;
use serde::{de::Visitor, Deserialize, Serialize};
use snafu::Snafu;

use crate::fs::paths::PathBufExt;

// Serde default functions
mod serde_default {
    pub const fn r#false() -> bool {
        false
    }

    pub fn r#ref() -> Option<String> {
        Some(String::from("HEAD"))
    }
}

#[derive(Debug, Snafu)]
pub enum ValidationError {}

pub struct Warnings<T: std::fmt::Display = String>(Vec<T>);

impl<T> Deref for Warnings<T>
where
    T: std::fmt::Display,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Warnings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Warnings<T>
where
    T: std::fmt::Display,
{
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn to_option(self) -> Option<Self> {
        match self.0.is_empty() {
            true => None,
            false => Some(self),
        }
    }

    pub fn inner(self) -> Vec<T> {
        self.0
    }
}

// TODO (@Techassi): Add config validation, because currently the filepaths used
// in various fields can be used for path traversal attacks.

/// Crumpet config representation.
///
/// The configuration is typically stored in the template target directory
/// under: `.crumpet/config.yml`.
// NOTE (@NickLarsenNZ): The directory shouldn't be mentioned in the docs here,
// that is the main bin will be responsible for setting the default location(s).
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    /// The configuration schema semantic version.
    ///
    /// This is especially used to handle breaking changes across versions.
    pub version: Version,

    /// Configuration pertaining to the template source.
    ///
    /// Example:
    ///
    /// ```yaml
    /// template:
    ///   source: https://github.com/my-org/my-template
    ///   ref: v1.0.1
    /// ```
    pub template: TemplateConfig,

    /// Configuration pertaining to raising Pull/Merge Requests.
    ///
    /// NOTE: This will only be used if `CI=true` (as is common in Github Actions and Gitlab CI/CD).
    pub pull_request: Option<PullRequestConfig>,
}

impl Config {
    pub fn validate(&self) -> Result<Option<Warnings>, ValidationError> {
        let mut warnings = Warnings::new();

        if let Some(w) = self.template.validate()? {
            warnings.extend(w.inner())
        }

        if let Some(pull_request) = &self.pull_request {
            if let Some(w) = pull_request.validate()? {
                warnings.extend(w.inner())
            }
        }

        Ok(Some(warnings))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TemplateConfig {
    /// The location of the template source.
    ///
    /// This can be a local directory path, or a git repository URL (https/ssh).
    /// For example, all of these are valid sources:
    ///
    /// ```yaml
    /// source: path/to/template
    /// source: https://github.com/my-org/my-template
    /// source: ssh://github.com:my-org/my-template?path=custom/template_dir
    /// ```
    ///
    /// Also see [`SourceIdentifier`].
    pub source: SourceIdentifier,

    /// A [committish] to refer to a commit, tag, or branch.
    ///
    /// If left unset, the repository's default branch (`HEAD`) will be used.
    /// NOTE: only used if the [`TemplateConfig::source`] is a git repository.
    ///
    /// [committish]: https://git-scm.com/docs/gitglossary#Documentation/gitglossary.txt-aiddefcommit-ishacommit-ishalsocommittish
    #[serde(
        rename = "ref",
        default = "serde_default::r#ref",
        skip_serializing_if = "Option::is_none"
    )]
    pub reference: Option<String>,
}

impl TemplateConfig {
    fn validate(&self) -> Result<Option<Warnings>, ValidationError> {
        let mut warnings = Warnings::new();

        if self.source.scheme == Scheme::File {
            let path = self.source.path();

            if !path.is_absolute() {
                warnings.push(format!(
                    "{path} is not an absolute path",
                    path = path.display()
                ))
            }
        }

        Ok(warnings.to_option())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PullRequestConfig {
    // NOTE (@NickLarsenNZ): We should think about whether this defaults to true
    // when CI=true.
    enabled: bool,

    /// Mark the pull request as a draft (default: false)
    #[serde(default = "serde_default::r#false")]
    draft: bool,

    /// Set the contents of the pull request title
    title: PullRequestTemplateSource,

    /// Set the contents of the pull request body
    body: PullRequestTemplateSource,

    /// Provide any number of labels / tags to be attached to the pull request
    #[serde(alias = "tags", skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<String>>,

    /// Provide any number of assignees
    #[serde(skip_serializing_if = "Option::is_none")]
    assignees: Option<Vec<String>>,
}

impl PullRequestConfig {
    fn validate(&self) -> Result<Option<Warnings>, ValidationError> {
        Ok(None)
    }
}

/// The location of the template source.
///
/// This can be a local directory path, or a git repository URL (https/ssh).
/// For example, all of these are valid sources:
///
/// ```yaml
/// source: path/to/template
/// source: https://github.com/my-org/my-template
/// source: ssh://github.com:my-org/my-template?path=custom/template_dir
/// ```
#[derive(Debug, PartialEq)]
pub struct SourceIdentifier(Url);

impl Deref for SourceIdentifier {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for SourceIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SourceIdentifierVisitor;

        impl<'de> Visitor<'de> for SourceIdentifierVisitor {
            type Value = SourceIdentifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid git url/path")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let url = Url::try_from(v).map_err(|err| serde::de::Error::custom(err))?;
                Ok(SourceIdentifier(url))
            }
        }

        deserializer.deserialize_str(SourceIdentifierVisitor)
    }
}

impl Serialize for SourceIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self.0))
    }
}

impl SourceIdentifier {
    pub fn path(&self) -> PathBuf {
        PathBuf::from(self.path.to_string())
    }
}

/// Content for the Pull Request template title and body.
///
/// This can either be a file, or an inline string.
///
/// ```yaml
/// title: Pull Request title
/// body: path/to/template/from/the/template.md # if you want to use a common template across all repositories
/// ```
///
/// Or:
///
/// ```yaml
/// title: chore: Template Update {{ template.ref }}
/// body: |
///   # Template Update
///
///   This change was generated by {{ template.source }}@{{ template.ref }}
/// ```
// TODO (@NickLarsenNZ): Allow template variables to be used so the content can be dynamic?
#[derive(Debug, PartialEq)]
pub enum PullRequestTemplateSource {
    /// Inline content template
    Template(String),

    /// Content template from a file
    File(PathBuf),
}

impl Serialize for PullRequestTemplateSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            PullRequestTemplateSource::Template(string) => serializer.serialize_str(&string),
            PullRequestTemplateSource::File(path) => {
                let path = path.to_str().ok_or(serde::ser::Error::custom(
                    "path contains invalid UTF-8 characters",
                ))?;
                serializer.serialize_str(path)
            }
        }
    }
}

impl<'de> Deserialize<'de> for PullRequestTemplateSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PullRequestTemplateSourceVisitor;

        impl<'de> Visitor<'de> for PullRequestTemplateSourceVisitor {
            type Value = PullRequestTemplateSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid inline template string or path to template file")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.is_empty() {
                    return Ok(PullRequestTemplateSource::Template(v.to_owned()));
                }

                // TODO (@Techassi): Remove unwrap
                let abs_path = PathBuf::from(v).absolutize().unwrap();

                match std::fs::metadata(abs_path).map(|meta| meta.is_file()) {
                    Ok(is_file) if is_file => Ok(PullRequestTemplateSource::File(PathBuf::from(v))),
                    Ok(is_file) if !is_file => {
                        Err(serde::de::Error::custom("template path is not a file"))
                    }
                    _ => Ok(PullRequestTemplateSource::Template(v.to_owned())),
                }
            }
        }

        deserializer.deserialize_str(PullRequestTemplateSourceVisitor)
    }
}

#[cfg(test)]
mod test {
    use gix::Url;

    use super::*;

    #[test]
    fn roundtrip() {
        let original = Config {
            version: Version::new(0, 0, 1),
            template: TemplateConfig {
                source: SourceIdentifier(
                    Url::from_bytes("https://github.com/my-org/my-template".into()).unwrap(),
                ),
                reference: Some(String::from("abcdef0")),
            },
            pull_request: Some(PullRequestConfig {
                enabled: true,
                draft: false,
                title: PullRequestTemplateSource::File("../../fixtures/file_01.tera".into()),
                body: PullRequestTemplateSource::Template(
                    "chore: Update templated files ({{ ref }})".into(),
                ),
                labels: Some(vec!["size/s".into()]),
                assignees: None,
            }),
        };

        if let Some(warnings) = original.validate().expect("valid config") {
            assert!(warnings.is_empty())
        }

        let yaml = serde_yaml::to_string(&original).unwrap();
        let copy: Config = serde_yaml::from_str(&yaml).unwrap();

        println!("{yaml}");

        assert_eq!(original, copy);
    }

    #[test]
    fn warnings() {
        let original = Config {
            version: Version::new(0, 0, 1),
            template: TemplateConfig {
                source: SourceIdentifier(Url::from_bytes("../Cargo.toml".into()).unwrap()),
                reference: Some(String::from("abcdef0")),
            },
            pull_request: Some(PullRequestConfig {
                enabled: true,
                draft: false,
                title: PullRequestTemplateSource::File("../../fixtures/file_01.tera".into()),
                body: PullRequestTemplateSource::Template(
                    "chore: Update templated files ({{ ref }})".into(),
                ),
                labels: Some(vec!["size/s".into()]),
                assignees: None,
            }),
        };

        if let Some(warnings) = original.validate().expect("valid config") {
            assert!(!warnings.is_empty());

            for warning in warnings.inner() {
                println!("warning: {warning}");
            }
        }

        let yaml = serde_yaml::to_string(&original).unwrap();
        let copy: Config = serde_yaml::from_str(&yaml).unwrap();

        println!("{yaml}");

        assert_eq!(original, copy);
    }

    #[test]
    fn path() {
        let url = Url::try_from("../Cargo.toml").unwrap();
        println!("{url:?}");
    }
}
