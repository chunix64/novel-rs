use std::path::Path;
use tokio::fs::{self, File};

use crate::config::sites::Site;

pub async fn init_evironment(sites: &Vec<Site>, database_url: &String) {
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

    for site in sites {
        let site_path = Path::new(&database_url).join(&format!("{}.sqlite3", site.db_name));
        if !site_path.exists() {
            File::create(&site_path).await.unwrap();
        }
    }

    // Need refactor, create master.sqlite3 (duplicate code)
    let site_path = Path::new(&database_url).join("master.sqlite3");
    if !site_path.exists() {
        File::create(&site_path).await.unwrap();
    }
}
