use std::path::PathBuf;

use semver::Version;
use serde::{de::Visitor, Deserialize, Serialize};

const fn r#false() -> bool {
    false
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub version: Version,
    pub template: TemplateConfig,
    pub pull_request: PullRequestConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TemplateConfig {
    pub source: SourceIdentifier,

    #[serde(default = "default_template_directory")]
    pub template_directory: PathBuf,

    #[serde(
        rename = "ref",
        default = "default_ref",
        skip_serializing_if = "Option::is_none"
    )]
    pub reference: Option<String>,
}

fn default_template_directory() -> PathBuf {
    PathBuf::from("template")
}

fn default_ref() -> Option<String> {
    Some(String::from("HEAD"))
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PullRequestConfig {
    enabled: bool,

    /// Mark the pull request as a draft (default: false)
    #[serde(default = "r#false")]
    draft: bool,

    /// Set the contents of the pull request title
    title: TemplateSource,

    /// Set the contents of the pull request body
    body: TemplateSource,

    /// Provide any number of labels / tags to be attached to the pull request
    #[serde(alias = "tags", skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<String>>,

    /// Provide any number of assignees
    #[serde(skip_serializing_if = "Option::is_none")]
    assignees: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub enum SourceIdentifier {
    File(PathBuf),
    Git(gix::Url),
}

impl Serialize for SourceIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SourceIdentifier::File(path) => {
                let path = path.to_str().ok_or(serde::ser::Error::custom(
                    "path contains invalid UTF-8 characters",
                ))?;
                serializer.serialize_str(path)
            }
            SourceIdentifier::Git(raw_url) => {
                let url = raw_url.to_bstring().to_string();
                serializer.serialize_str(&url)
            }
        }
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
                formatter.write_str("a local file path or a remote git repository url")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // NOTE (@Techassi): Sadly the find_scheme function of gix is
                // private and as such we cannot use that to check IF we should
                // try to parse the input as a git url. That's the reason why
                // we "brute-force" the parsing first, and then fall back to
                // parsing the input as a local path.
                match gix::Url::try_from(v) {
                    Ok(url) => Ok(SourceIdentifier::Git(url)),
                    Err(_) => Ok(SourceIdentifier::File(PathBuf::from(v))),
                }
            }
        }

        deserializer.deserialize_str(SourceIdentifierVisitor)
    }
}

// TODO (@Techassi): To make the in-place string and the variants work, we need our own serialize and deserialize
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateSource {
    Template(String),

    #[serde(rename = "template_file")]
    File(PathBuf),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn roundtrip() {
        let original = Config {
            version: Version::new(0, 0, 1),
            template: TemplateConfig {
                source: SourceIdentifier::Git(
                    gix::Url::from_bytes("https://github.com/my-org/my-template".into()).unwrap(),
                ),
                template_directory: PathBuf::from("template"),
                reference: Some(String::from("abcdef0")),
            },
            pull_request: PullRequestConfig {
                enabled: true,
                draft: false,
                title: TemplateSource::Template("".into()),
                body: TemplateSource::Template("".into()),
                labels: Some(vec!["size/s".into()]),
                assignees: None,
            },
        };

        let yaml = serde_yaml::to_string(&original).unwrap();
        let copy: Config = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(original, copy);
    }
}
