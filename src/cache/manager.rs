use crate::utils::string::get_valid_file_name;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, error, trace};

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
            match fs::create_dir_all(&save_dir).await {
                Ok(_) => {
                    debug!(target = %"cache", directory = %save_dir.display(), "Created cache directory")
                }
                Err(error) => {
                    error!(target = %"cache", directory = %save_dir.display(), ?error, "Failed to create cache directory")
                }
            }
        }
        let file_path = save_dir.join(file_name);
        match fs::write(&file_path, data).await {
            Ok(_) => {
                debug!(target = %"cache", file_path = %file_path.display(), kind = %"save", "Cache saved")
            }
            Err(error) => {
                error!(target = %"cache", file_path = %file_path.display(), kind = %"save", ?error,"Failed to save cache")
            }
        };
    }

    pub async fn load<P: AsRef<Path>>(&self, sub_path: P, file_name: &str) -> Option<String> {
        let file_name = get_valid_file_name(file_name);
        let file_path = self.path.join(sub_path).join(file_name);
        if fs::metadata(&file_path).await.is_err() {
            trace!(target = %"cache", file_path = %file_path.display(), kind = %"load", "Cache file does not exist");
            return None;
        }

        match fs::read_to_string(&file_path).await {
            Ok(content) => {
                debug!(target = %"cache", file_path = %file_path.display(), kind = %"load", "Cache loaded");
                Some(content)
            }
            Err(error) => {
                error!(target = %"cache", file_path = %file_path.display(), kind = %"load", ?error, "Failed to read cache");
                None
            }
        }
    }

    pub async fn is_exists<P: AsRef<Path>>(&self, sub_path: P, file_name: &str) -> bool {
        let file_name = get_valid_file_name(file_name);
        let file_path = self.path.join(sub_path).join(file_name);
        trace!(target = %"cache", file_path = %file_path.display(), kind = %"check", "Checked cache existence");
        fs::metadata(&file_path).await.is_ok()
    }
}
