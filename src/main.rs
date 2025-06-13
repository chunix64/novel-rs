use clap::Parser;

mod cache;
mod config;
mod core;
mod db;
mod init;
mod service;
mod site;
mod utils;
use config::{app::AppConfig, cli::Cli, sites::SiteEnum};
use core::cli::handle_cli;
use db::Database;
use init::environment::init_environment;
use init::logs::init_logs;

#[tokio::main()]
async fn main() {
    init_logs();
    let cli = Cli::parse();
    let sites = SiteEnum::get_site();
    let pool = init_environment(&sites, &cli).await;
    let database = Database::new(pool);
    let app_config = AppConfig::new(cli.delay_min, cli.delay_max, cli.no_cache, &cli.data_path);
    handle_cli(&cli, database, app_config).await;
}
