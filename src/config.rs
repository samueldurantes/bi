/// This enum describes the config parameters that application need to work.
#[derive(clap::Parser, Clone)]
pub struct Config {
    /// This is a connection URL that Postgres must use.
    #[clap(long, env)]
    pub database_url: String,

    /// This is the HTTP port that server will to run
    #[clap(long, env)]
    pub port: u16,
}
