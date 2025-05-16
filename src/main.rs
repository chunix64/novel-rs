use clap::Parser;

mod config;
mod db;
mod utils;
use config::cli::Cli;
use config::sites::get_sites;
use db::DB;
use utils::env::init_environment;

#[tokio::main()]
async fn main() {
    let cli = Cli::parse();
    let sites = get_sites();
    let pool = init_environment(&sites, &cli.database_url).await;

    let db = DB::new(pool);
    db.author.insert("Le Trung Ky").await.unwrap();
    println!("Test: {:#?}", db.author.get_all().await);

    if cli.no_cache {
        println!("no cache");
    }
}
