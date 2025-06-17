use tracing::error;

use crate::{
    config::{
        app::AppConfig,
        cli::Cli,
        sites::{ServiceEnum, SiteEnum},
    },
    db::Database,
};

pub async fn handle_cli(cli: &Cli, database: Database, app_config: AppConfig) {
    let site_enum = match SiteEnum::from_str(&cli.site) {
        Some(site_enum) => site_enum,
        None => {
            error!(target = %"core", site = %cli.site ,"Get site error");
            std::process::exit(1);
        }
    };
    match site_enum.create_service(database, app_config) {
        ServiceEnum::Novel(service) => {
            if cli.sync_items {
                service.sync_novels().await;
            }
            if cli.sync_contents {
                service.sync_all_novel_chapters().await;
            }
        }
    }
}
