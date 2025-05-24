pub struct AppConfig {
    delay_min: u64,
    delay_max: u64,
    cache: bool,
    data_path: String,
}

impl AppConfig {
    pub fn new(delay_min: u64, delay_max: u64, cache: bool, data_path: &str) -> Self {
        let mut delay_min = delay_min;
        if delay_max < delay_min {
            println!("Delay max should be greater or equal Delay min!");
            delay_min = delay_max;
            println!("So Delay min = Delay max now!");
        }
        Self {
            delay_min,
            delay_max,
            cache,
            data_path: data_path.to_string(),
        }
    }

    pub fn delay_min(&self) -> u64 {
        self.delay_min
    }

    pub fn delay_max(&self) -> u64 {
        self.delay_max
    }

    pub fn is_cache(&self) -> bool {
        self.cache
    }

    pub fn data_path(&self) -> String {
        self.data_path.clone()
    }
}
