use clap::Parser;

mod config;
mod db;
mod utils;
use config::cli::Cli;
use config::sites::get_sites;
use utils::env::init_evironment;


#[tokio::main()]
async fn main(){
    let cli = Cli::parse();
    let sites = get_sites();
    init_evironment(&sites, &cli.database_url).await;
    if cli.no_cache {
        println!("no cache");
    }
}
