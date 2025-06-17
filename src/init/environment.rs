use std::path::Path;
use tokio::fs::{self, File};
use tracing::{debug, error, info};

use crate::{
    config::{cli::Cli, sites::SiteEnum},
    db::schema::init_db,
};

pub async fn init_environment(sites: &[SiteEnum], cli: &Cli) -> sqlx::SqlitePool {
    let database_url = "data/db";

    create_folders().await;
    create_databases(sites, database_url).await;

    let master_path = Path::new(&database_url);
    let master_database_name = "master";

    create_database(master_path, master_database_name).await;

    let pool_name = match SiteEnum::from_str(&cli.site) {
        Some(site) => site.database_name(),
        None => master_database_name,
    };

    let db_path = format!("sqlite://{}/sites/{}.sqlite3", &database_url, &pool_name);

    info!(target = %"init", database = %db_path, "Connecting to main database");
    let pool = match sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&db_path)
        .await
    {
        Ok(pool) => {
            debug!(target = %"init", database = %db_path, "Connected to main database");
            pool
        }
        Err(error) => {
            error!(target = %"init", database = %db_path, ?error, "Failed to connect to main database");
            std::process::exit(1);
        }
    };
    match init_db(&pool).await {
        Ok(_) => debug!(target = %"init", database = %db_path, "initialized database"),
        Err(error) => {
            error!(target = %"init", database = %db_path, ?error, "Failed to initalize database")
        }
    };

    info!(target = %"init", site = %pool_name, "initialized environment");
    pool
}

async fn create_folders() {
    const DIRECTORIES: &[&str] = &[
        "data/db/sites",
        "data/archives",
        "data/logs",
        "data/cache",
        "data/media/images",
        "data/media/videos",
        "data/media/audio",
    ];

    for directory in DIRECTORIES {
        if !Path::new(&directory).exists() {
            match fs::create_dir_all(Path::new(&directory)).await {
                Ok(_) => debug!(target = %"init", %directory, "Created directory"),
                Err(error) => {
                    error!(target = %"init", %directory, ?error, "Failed create directory")
                }
            };
        }
    }
}

async fn create_databases(sites: &[SiteEnum], database_url: &str) {
    let database_path = Path::new(&database_url).join("sites");
    for site in sites {
        create_database(&database_path, site.database_name()).await;
    }
}

async fn create_database(base_path: &Path, database_name: &str) {
    let database_path = base_path.join(format!("{}.sqlite3", database_name));
    let db_uri = format!("sqlite://{}", database_path.display());
    if !database_path.exists() {
        match File::create(&database_path).await {
            Ok(_) => {
                debug!(target = %"init", database = %database_path.display(),"Created database file");
            }
            Err(error) => {
                debug!(target = %"init", database = %database_path.display(), ?error, "Failed created database file");
                return;
            }
        };
        match init_database(&db_uri).await {
            Ok(_) => debug!(target = %"init", database = %db_uri, "initialized database"),
            Err(error) => {
                error!(target = %"init", database = %db_uri, ?error, "Failed to initalize database")
            }
        };
    }
}

async fn init_database(database_url: &str) -> Result<sqlx::SqlitePool, sqlx::Error> {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(database_url)
        .await?;
    init_db(&pool).await?;
    Ok(pool)
}
