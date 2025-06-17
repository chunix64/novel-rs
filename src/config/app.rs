use tracing::warn;

pub struct AppConfig {
    delay_min: u64,
    delay_max: u64,
    no_cache: bool,
    data_path: String,
}

impl AppConfig {
    pub fn new(
        delay_min: u64,
        delay_max: u64,
        no_cache: bool,
        data_path: impl Into<String>,
    ) -> Self {
        let mut delay_min = delay_min;
        if delay_max < delay_min {
            warn!(
                target = "config",
                %delay_min,
                %delay_max,
                "Invalid delay config: delay_max ({delay_max}) < delay_min ({delay_min}); forcing delay_min = {delay_max}"
            );
            delay_min = delay_max;
        }
        Self {
            delay_min,
            delay_max,
            no_cache,
            data_path: data_path.into(),
        }
    }

    pub fn delay_min(&self) -> u64 {
        self.delay_min
    }

    pub fn delay_max(&self) -> u64 {
        self.delay_max
    }

    pub fn is_cache(&self) -> bool {
        !self.no_cache
    }

    pub fn data_path(&self) -> &str {
        &self.data_path
    }
}
