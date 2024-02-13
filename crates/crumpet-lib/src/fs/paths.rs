use std::{env, path::PathBuf};

use snafu::{OptionExt, ResultExt, Snafu};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("invalid path {}", path.display()))]
    InvalidPath { path: PathBuf },

    #[snafu(display("invalid current working directory"))]
    InvalidCurrentWorkingDirectory { source: std::io::Error },
}

pub trait PathBufExt: Sized {
    fn into_str<'a>(&'a self) -> Result<&'a str>;

    /// Returns any provided `path` as an absolute path. If the provided `path`
    /// is already absolute, it is returned as is.
    fn absolutize(self) -> Result<Self>;

    fn remove_keyword(self) -> Result<Self>;
}

impl PathBufExt for PathBuf {
    fn into_str<'a>(&'a self) -> Result<&'a str> {
        self.to_str().context(InvalidPathSnafu { path: self })
    }

    fn absolutize(self) -> Result<PathBuf> {
        if self.is_absolute() {
            return Ok(self);
        }

        let cwd = env::current_dir().context(InvalidCurrentWorkingDirectorySnafu)?;
        let path = cwd.join(self);

        Ok(path)
    }

    fn remove_keyword(self) -> Result<Self> {
        let parent = self.parent();

        let file_name = self
            .file_name()
            .context(InvalidPathSnafu { path: self.clone() })?
            .to_str()
            .context(InvalidPathSnafu { path: self.clone() })?;

        file_name.replace("tera", "");
        todo!()
    }
}

pub fn pathbuf_to_str<'a>(path: &'a PathBuf) -> Result<&'a str> {
    path.to_str()
        .context(InvalidPathSnafu { path: path.clone() })
}
