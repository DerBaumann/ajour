#[derive(Debug, clap::Parser)]
pub struct Config {
    #[arg(long, env)]
    pub database_url: String,
    #[arg(long, short, env, default_value_t = 8080)]
    pub port: u16,
}
