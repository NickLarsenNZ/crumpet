use cli::Cli;

mod cli;

#[tokio::main]
async fn main() {
    let app = Cli::new();

    // ...

    let _ = app.run().await;
}
