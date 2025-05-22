use crate::{db::Database, service::novel::NovelService, site::docln::provider::DoclnProvider};

use super::cli::Cli;

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

    pub fn create_service(&self, database: Database, cli: &Cli) -> ServiceEnum {
        match self {
            Self::Docln => {
                let provider = DoclnProvider;
                ServiceEnum::Novel(NovelService::new(provider, database, cli.delay))
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
