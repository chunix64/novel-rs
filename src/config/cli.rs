use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long, default_value_t = false)]
    pub cache: bool,

    #[arg(long, default_value_t = false)]
    pub sync_items: bool,

    #[arg(long, default_value_t = false)]
    pub sync_contents: bool,

    #[arg(long, default_value_t = false)]
    pub test: bool,

    #[arg(short, long, default_value = "docln")]
    pub site: String,

    #[arg(short, long, default_value = "data")]
    pub data_path: String,

    #[arg(long, default_value_t = 1000)]
    pub delay_min: u64,

    #[arg(long, default_value_t = 3000)]
    pub delay_max: u64,
}
