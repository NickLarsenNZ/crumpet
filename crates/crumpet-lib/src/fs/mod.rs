use std::path::PathBuf;

use snafu::{ResultExt, Snafu};
use tokio::fs;

use crate::fs::paths::PathBufExt;

pub mod paths;

const DEFAULT_MATCH_PATTERN: &str = "**/*.tera*";

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("invalid path"))]
    InvalidPath { source: paths::Error },

    #[snafu(display("invalid glob pattern"))]
    InvalidPattern { source: glob::PatternError },

    #[snafu(display("encountered unreadable path"))]
    UnreadablePath { source: glob::GlobError },

    #[snafu(display("failed to read file"))]
    ReadFile {
        source: std::io::Error,
        path: PathBuf,
    },
}

pub fn build_file_list(base_path: PathBuf) -> Result<Vec<PathBuf>> {
    let pattern = build_glob_pattern(base_path)?;

    let paths: Vec<PathBuf> = glob::glob(&pattern)
        .context(InvalidPatternSnafu)?
        .collect::<Result<_, _>>()
        .context(UnreadablePathSnafu)?;

    let paths = paths.into_iter().filter(|p| p.is_file()).collect();
    Ok(paths)
}

pub fn build_glob_pattern(base_path: PathBuf) -> Result<String> {
    let base_path = base_path.absolutize().context(InvalidPathSnafu)?;
    let path = base_path.join(DEFAULT_MATCH_PATTERN);
    Ok(path.into_str().context(InvalidPathSnafu)?.to_owned())
}

// NOTE (@Techassi): Is there much added value to this function compared to
// straight up calling the fs::read_to_string function?
pub async fn read_file(path: impl Into<PathBuf>) -> Result<String> {
    let path: PathBuf = path.into();

    // NOTE (@Techassi): Is this even required? Only if we explicitly want to
    // emit a specialized error variant when a file doesn't exist.
    fs::try_exists(&path)
        .await
        .context(ReadFileSnafu { path: path.clone() })?;

    fs::read_to_string(&path)
        .await
        .context(ReadFileSnafu { path })
}

pub async fn write_file(
    file_path: impl Into<PathBuf>,
    contents: impl AsRef<str>,
    replace_keyword: bool,
) -> Result<()> {
    let file_path: PathBuf = file_path.into();

    // let file_path = if replace_keyword {
    //     file_path.set_file_name(file_path.file_name().)
    // };

    // fs::write(file_path, contents).await

    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn fixtures_path() -> PathBuf {
        PathBuf::new()
            .join(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("fixtures")
    }

    #[rstest]
    fn file_list(fixtures_path: PathBuf) {
        let paths = build_file_list(fixtures_path).unwrap();
        println!("{paths:#?}");
        assert_eq!(paths.len(), 3)
    }
}
