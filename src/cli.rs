use clap::{Args, Parser, Subcommand};
use snafu::Snafu;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {}

#[derive(Debug, Parser)]
#[command(author, version, about, propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Commands,
}

impl Cli {
    pub async fn run(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Templating operations
    #[command(alias("tpl"))]
    Template(TemplateArgs),
}

#[derive(Debug, Args)]
pub struct TemplateArgs {
    #[command(subcommand)]
    pub subcommand: TemplateCommands,
}

#[derive(Debug, Subcommand)]
pub enum TemplateCommands {
    /// Render the specified template
    #[command()]
    Render(TemplateRenderArgs),
}

#[derive(Debug, Args)]
pub struct TemplateRenderArgs {
    /// Template souce
    /// Eg: https://github.com/example/template
    #[arg()]
    source: String,

    /// Template render destination
    /// Eg: ./
    #[arg()]
    destination: String,
}
