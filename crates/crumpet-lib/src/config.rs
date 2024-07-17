use std::path::PathBuf;

use semver::Version;
use serde::{Deserialize, Serialize};

const fn r#false() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub version: Version,
    pub template: TemplateConfig,
    pub pull_request: PullRequestConfig,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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
    #[serde(alias = "tags", skip_serializing_if = "Vec::is_empty")]
    labels: Vec<String>,

    /// Provide any number of assignees
    #[serde(skip_serializing_if = "Vec::is_empty")]
    assignees: Vec<String>,
}

// TODO (@Techassi): Implement our own deserialize
#[derive(Debug, Deserialize)]
pub enum SourceIdentifier {
    File(PathBuf),
    Git(String), // TODO (@Techassi): Make this a gix::Url instead
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
            SourceIdentifier::Git(url) => serializer.serialize_str(url),
        }
    }
}

// TODO (@Techassi): To make the in-place string and the variants work, we need our own serialize and deserialize
#[derive(Debug, Serialize, Deserialize)]
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
    fn example_config() {
        let config = Config {
            version: Version::new(0, 0, 1),
            template: TemplateConfig {
                source: SourceIdentifier::Git("https://github.com/my-org/my-template".into()),
                template_directory: PathBuf::from("template"),
                reference: Some(String::from("abcdef0")),
            },
            pull_request: PullRequestConfig {
                enabled: true,
                draft: false,
                title: TemplateSource::Template("".into()),
                body: TemplateSource::Template("".into()),
                labels: vec!["size/s".into()],
                assignees: vec![],
            },
        };

        let yaml = serde_yaml::to_string(&config).unwrap();
        println!("{yaml}");
    }
}
