use crate::{
    config::{
        app::AppConfig,
        cli::Cli,
        sites::{ServiceEnum, SiteEnum},
    },
    db::Database,
};

pub async fn handle_cli(cli: &Cli, database: Database, app_config: AppConfig) {
    let service_enum = SiteEnum::from_str(&cli.site).unwrap();
    match service_enum.create_service(database, app_config) {
        ServiceEnum::Novel(service) => {
            if cli.sync_items {
                service.sync_novels().await;
            }
            if cli.sync_contents {
                service.sync_all_novel_chapters().await;
            }
            if cli.test {
                service.test().await;
            }
        }
    }
}
