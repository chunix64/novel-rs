use clap::Parser;

mod config;
mod db;
mod site;
mod utils;
use config::cli::Cli;
use config::sites::get_sites;
use site::docln::provider::DoclnProvider;
use site::provider_base::ContentProvider;
use utils::env::init_environment;

#[tokio::main()]
async fn main() {
    let cli = Cli::parse();
    let sites = get_sites();
    let _pool = init_environment(&sites, &cli.database_url).await;

    if cli.no_cache {
        println!("no cache");
    }

    let docln = DoclnProvider;
    println!("{:#?}", docln.get_items().await);
}
