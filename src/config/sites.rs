use crate::{db::Database, service::novel::NovelService, site::docln::provider::DoclnProvider};

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
                let provider = DoclnProvider::new(ProviderConfig::from(&app_config));
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
