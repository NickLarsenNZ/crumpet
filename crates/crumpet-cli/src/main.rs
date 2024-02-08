use clap::Parser;
use cli::Cli;

mod cli;

#[tokio::main]
async fn main() {
    let app = Cli::parse();

    // ...

    let _ = app.run().await;
}
