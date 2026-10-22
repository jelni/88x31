use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// host to bin to
    #[arg(long, default_value = "localhost")]
    pub host: String,
    /// port to bind to
    #[arg(long, default_value_t = 8080)]
    pub port: u16,
}
