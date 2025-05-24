use std::path::PathBuf;

use crate::{
    cache::manager::CacheManager, db::Database, service::novel::NovelService,
    site::docln::provider::DoclnProvider,
};

use super::{app::AppConfig, provider::ProviderConfig};

pub enum SiteEnum {
    Docln,
}

pub enum ServiceEnum {
    Novel(NovelService),
}

impl SiteEnum {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "docln" => Some(Self::Docln),
            _ => None,
        }
    }

    pub fn create_service(&self, database: Database, app_config: AppConfig) -> ServiceEnum {
        match self {
            Self::Docln => {
                let cache_path = PathBuf::from(app_config.data_path()).join("cache");
                let cache_manager = CacheManager::new(&cache_path);
                let provider = DoclnProvider::new(ProviderConfig::from(&app_config), cache_manager);
                ServiceEnum::Novel(NovelService::new(provider, database, app_config))
            }
        }
    }

    pub fn database_name(&self) -> String {
        match self {
            Self::Docln => "docln".to_string(),
        }
    }

    pub fn get_site() -> Vec<Self> {
        vec![Self::Docln]
    }
}
