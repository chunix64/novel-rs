use super::app::AppConfig;

pub struct ProviderConfig {
    delay_min: u64,
    delay_max: u64,
    is_cache: bool,
}

impl ProviderConfig {
    pub fn delay_min(&self) -> u64 {
        self.delay_min
    }

    pub fn delay_max(&self) -> u64 {
        self.delay_max
    }

    pub fn is_cache(&self) -> bool {
        self.is_cache
    }
}

impl From<AppConfig> for ProviderConfig {
    fn from(app_config: AppConfig) -> Self {
        Self {
            delay_min: app_config.delay_min(),
            delay_max: app_config.delay_max(),
            is_cache: app_config.is_cache(),
        }
    }
}

impl From<&AppConfig> for ProviderConfig {
    fn from(app_config: &AppConfig) -> Self {
        Self {
            delay_min: app_config.delay_min(),
            delay_max: app_config.delay_max(),
            is_cache: app_config.is_cache(),
        }
    }
}
