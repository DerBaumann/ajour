// TODO: Add default port

#[derive(Debug, clap::Parser)]
pub struct Config {
    #[arg(long, env)]
    pub database_url: String,
    #[arg(long, short, env)]
    pub port: u16,
}
