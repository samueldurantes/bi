use bi::{config::Config, http};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let config = Config::parse();

    http::serve(config).await?;

    Ok(())
}
