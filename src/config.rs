/// This enum describes the config parameters that application need to work.
#[derive(clap::Parser, Clone)]
pub struct Config {
    /// This is a connection URL that Postgres must use.
    #[clap(long, env)]
    pub database_url: String,

    /// This is the HTTP port that server will to run.
    #[clap(long, env)]
    pub port: u16,

    /// This is the interval at which node data will be updated in the database.
    #[clap(long, env)]
    pub pooling_inverval_in_seconds: u64,
}
