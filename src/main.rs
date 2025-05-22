use clap::Parser;

mod config;
mod core;
mod db;
mod service;
mod site;
mod utils;
use config::{app::AppConfig, cli::Cli, sites::SiteEnum};
use core::cli::handle_cli;
use db::Database;
use utils::env::init_environment;

#[tokio::main()]
async fn main() {
    let cli = Cli::parse();
    let sites = SiteEnum::get_site();
    let pool = init_environment(&sites, &cli).await;
    let database = Database::new(pool);
    let app_config = AppConfig::new(cli.delay_min, cli.delay_max);
    handle_cli(cli, database, app_config).await;
}
