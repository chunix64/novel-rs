use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

pub fn current_stamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub async fn sleep_random_range(min: u64, max: u64) {
    assert!(
        max >= min,
        "Delay max should be greater or equal Delay min!"
    );
    let milis = rand::random_range(min..=max);
    println!("Sleep: {}ms in ({}-{})", milis, min, max);
    sleep(Duration::from_millis(milis)).await;
}

pub fn calculate_logarithm_delay(
    base: u64,
    attempt: u64,
    min: u64,
    max: u64,
    log_base: f64,
) -> u64 {
    // anti blocking algorithm
    // y = n * log_b(n) * x
    // with y = delay time
    // with x = base delay time
    // and n = attempt
    // and b = log base
    let growth_factor: f64 = attempt as f64 * (attempt as f64).log(log_base);
    ((base as f64 * growth_factor) as u64).clamp(min, max)
}

pub fn calculate_exponent_delay(delay: u64, exponent_base: u32) -> u64 {
    delay.pow(exponent_base)
}

pub fn calculate_hybrid_delay(
    base: u64,
    attempt: u64,
    min: u64,
    max: u64,
    log_base: f64,
    exponent_base: u32,
) -> u64 {
    let logarithm_delay = calculate_logarithm_delay(base, attempt, min, max, log_base);
    if logarithm_delay > 500 {
        return logarithm_delay;
    } else {
        return calculate_exponent_delay(logarithm_delay, exponent_base).clamp(min, max);
    }
}
