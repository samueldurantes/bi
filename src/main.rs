use anyhow::Context;
use bi::{config::Config, http};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let config = Config::parse();
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("Error when trying to connect to database")?;

    sqlx::migrate!().run(&db).await?;
    http::serve(config, db).await?;

    Ok(())
}
