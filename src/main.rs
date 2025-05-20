use clap::Parser;

mod config;
mod core;
mod db;
mod service;
mod site;
mod utils;
use config::{cli::Cli, sites::SiteEnum};
use core::cli::handle_cli;
use db::Database;
use utils::env::init_environment;

#[tokio::main()]
async fn main() {
    let cli = Cli::parse();
    let sites = SiteEnum::get_site();
    let pool = init_environment(&sites, &cli).await;
    let database = Database::new(pool);
    handle_cli(cli, database).await;
}
