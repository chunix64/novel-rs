use clap::Parser;

mod config;
use config::cli::Cli;

fn main(){
    let cli = Cli::parse();
    if cli.no_cache {
        println!("no cache");
    }
}
