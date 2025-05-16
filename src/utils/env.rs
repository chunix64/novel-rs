use std::path::Path;
use tokio::fs::{self, File};

use crate::{config::sites::Site, db::schema::init_db};

pub async fn init_environment(sites: &Vec<Site>, database_url: &String) -> sqlx::SqlitePool {
    create_folders(&database_url).await;
    create_databases(&sites, &database_url).await;

    let master_path = Path::new(&database_url).join("master.sqlite3");
    if !master_path.exists() {
        File::create(&master_path).await.unwrap();
    }

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&format!("sqlite://{}/master.sqlite3", &database_url))
        .await
        .unwrap();
    init_db(&pool).await.unwrap();
    pool
}

async fn create_folders(database_url: &String) {
    let folders = [
        database_url,
        "data/archives",
        "data/logs",
        "data/cache",
        "data/media/images",
        "data/media/videos",
        "data/media/audio",
    ];

    for folder in folders {
        if !Path::new(&folder).exists() {
            fs::create_dir_all(Path::new(&folder)).await.unwrap();
        }
    }
}

async fn create_databases(sites: &Vec<Site>, database_url: &String) {
    for site in sites {
        let site_path = Path::new(&database_url).join(&format!("{}.sqlite3", site.db_name));
        let db_uri = format!("sqlite://{}", site_path.display());
        println!("path: {:#?}", site_path);
        if !site_path.exists() {
            File::create(&site_path).await.unwrap();
            init_database(&db_uri).await;
        }
    }
}

async fn init_database(database_url: &String) -> sqlx::SqlitePool {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(database_url)
        .await
        .unwrap();
    init_db(&pool).await.unwrap();
    pool
}
