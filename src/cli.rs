use snafu::Snafu;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {}

pub struct Cli {}

impl Cli {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self) -> Result<()> {
        Ok(())
    }
}
