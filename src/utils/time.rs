use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

pub fn current_stamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub async fn sleep_random_range(min: u64, max: u64) {
    assert!(max > min, "Delay max should be greater or equal Delay min!");
    let milis = rand::random_range(min..=max);
    println!("Sleep: {}ms", milis);
    sleep(Duration::from_millis(milis)).await;
}
