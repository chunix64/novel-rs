use clap::Parser;

mod config;
mod db;
mod service;
mod site;
mod utils;
use config::cli::Cli;
use config::sites::get_sites;
use db::Database;
use service::novel::NovelService;
use site::docln::provider::DoclnProvider;
use utils::env::init_environment;

#[tokio::main()]
async fn main() {
    let cli = Cli::parse();
    let sites = get_sites();
    let pool = init_environment(&sites, &cli.database_url).await;
    let database = Database::new(pool);
    let provider = DoclnProvider;

    if cli.no_cache {
        println!("no cache");
    }

    let novel_service = NovelService::new(provider, database);
    println!("Test: {:#?}", novel_service.test().await);
}
