use std::path::{Path, PathBuf};
use tokio::fs;
use crate::utils::string::get_valid_file_name;

pub struct CacheManager {
    path: PathBuf,
}

impl CacheManager {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }

    pub async fn save<P: AsRef<Path>>(&self, sub_path: P, file_name: &str, data: &str) {
        let save_dir = self.path.join(sub_path);
        let file_name = get_valid_file_name(file_name);
        if fs::metadata(&save_dir).await.is_err() {
            fs::create_dir_all(&save_dir).await.unwrap();
        }
        let file_path = save_dir.join(file_name);
        fs::write(&file_path, data).await.unwrap();
        println!("Cache saved: {}", &file_path.display());
    }

    pub async fn load<P: AsRef<Path>>(&self, sub_path: P, file_name: &str) -> Option<String> {
        let file_path = self.path.join(sub_path).join(file_name);
        if fs::metadata(&file_path).await.is_err() {
            return None;
        }

        let content = fs::read_to_string(file_path).await.unwrap();
        Some(content)
    }

    pub async fn is_exists<P: AsRef<Path>>(&self, sub_path: P, file_name: &str) -> bool {
        let file_path = self.path.join(sub_path).join(file_name);
        if fs::metadata(&file_path).await.is_err() {
            return false;
        } else {
            return true;
        }
    }
}
