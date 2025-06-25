use anyhow::Context;
use bi::{config::Config, http, synchronizer::synchronizer};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init();

    let config = Config::parse();
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("Error when trying to connect to database")?;

    // Spawns the synchronizer (hypocritical name LOL) task
    tokio::spawn(synchronizer(db.clone()));

    sqlx::migrate!().run(&db).await?;
    http::serve(config, db).await?;

    Ok(())
}
