use anyhow::Result;

use attic_client::cli;

#[tokio::main]
async fn main() -> Result<()> {
    init_logging()?;
    cli::run().await
}

fn init_logging() -> Result<()> {
    tracing_subscriber::fmt::init();
    Ok(())
}
