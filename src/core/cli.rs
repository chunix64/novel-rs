use crate::{
    config::{
        cli::Cli,
        sites::{ServiceEnum, SiteEnum},
    },
    db::Database,
};

pub async fn handle_cli(cli: Cli, database: Database) {
    let service_enum = SiteEnum::from_str(&cli.site).unwrap();
    match service_enum.create_service(database, &cli) {
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
