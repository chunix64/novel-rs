use super::app::AppConfig;

pub struct ProviderConfig {
    delay_min: u64,
    delay_max: u64,
}

impl ProviderConfig {
    pub fn new(delay_min: u64, delay_max: u64) -> Self {
        assert!(
            delay_max > delay_min,
            "Delay max should be greater or equal Delay min!"
        );
        Self {
            delay_min,
            delay_max,
        }
    }

    pub fn delay_min(&self) -> u64 {
        self.delay_min
    }

    pub fn delay_max(&self) -> u64 {
        self.delay_max
    }
}

impl From<AppConfig> for ProviderConfig {
    fn from(app_config: AppConfig) -> Self {
        Self {
            delay_min: app_config.delay_min(),
            delay_max: app_config.delay_max(),
        }
    }
}

impl From<&AppConfig> for ProviderConfig {
    fn from(app_config: &AppConfig) -> Self {
        Self {
            delay_min: app_config.delay_min(),
            delay_max: app_config.delay_max(),
        }
    }
}
