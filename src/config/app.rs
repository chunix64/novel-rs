pub struct AppConfig {
    delay_min: u64,
    delay_max: u64,
}

impl AppConfig {
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
