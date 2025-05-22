use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long, default_value_t = false)]
    pub no_cache: bool,

    #[arg(long, default_value_t = false)]
    pub sync_items: bool,

    #[arg(long, default_value_t = false)]
    pub sync_contents: bool,

    #[arg(long, default_value = "data/db")]
    pub database_url: String,

    #[arg(short, long, default_value = "docln")]
    pub site: String,

    #[arg(long, default_value_t = 0)]
    pub delay_min: u64,

    #[arg(long, default_value_t = 500)]
    pub delay_max: u64,
}
