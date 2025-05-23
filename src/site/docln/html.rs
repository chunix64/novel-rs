use crate::utils::{
    http::fetch_url,
    time::{calculate_hybrid_delay, sleep_random_range},
};

pub async fn fetch_novels(index: i64) -> (u16, Option<String>) {
    let url = format!("https://docln.net/danh-sach?page={}", index);
    let html = fetch_url(&url).await;
    let status_code = html.status().as_u16();
    let content = html.text().await.ok();
    (status_code, content)
}

pub async fn fetch_novels_retry(
    index: i64,
    sleep_min: u64,
    sleep_max: u64,
    max_retry: Option<u64>,
) -> Option<String> {
    let html: Option<String> =
        fetch_with_retry(|| fetch_novels(index), sleep_min, sleep_max, max_retry).await;
    html
}

pub async fn fetch_chapters(slug: &str) -> (u16, Option<String>) {
    let url = format!("https://docln.net{}", slug);
    let html = fetch_url(&url).await;
    let status_code = html.status().as_u16();
    let content = html.text().await.ok();
    (status_code, content)
}

pub async fn fetch_chapters_retry(
    slug: &str,
    sleep_min: u64,
    sleep_max: u64,
    max_retry: Option<u64>,
) -> Option<String> {
    let html: Option<String> =
        fetch_with_retry(|| fetch_chapters(slug), sleep_min, sleep_max, max_retry).await;
    html
}

pub async fn fetch_with_retry<F, Fut>(
    mut fetch_fn: F,
    sleep_min: u64,
    sleep_max: u64,
    max_retry: Option<u64>,
) -> Option<String>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = (u16, Option<String>)>,
{
    let mut attempt = 1;
    // sleep should be never <= 0 and max should be > min
    let sleep_min = sleep_min.max(1);
    let sleep_max = sleep_max.max(2);
    let html: Option<String> = loop {
        let (status_code, html) = fetch_fn().await;
        if status_code == 200 {
            if let Some(content) = html {
                break Some(content);
            } else {
                println!("STATUS OK but has no html");
                break None;
            }
        }

        println!("Failed attempt: {}", attempt);

        if let Some(max) = max_retry {
            // sleep_rate is same with attempt
            if attempt > max {
                break None;
            }
        }

        // anti blocking algorithm
        // y = n * log_2(n) * x
        // with y = delay time
        // with x = base delay time
        // and n = attempt
        let current_min = calculate_hybrid_delay(sleep_min, attempt, 1, 5000, 2.0, 2);
        let current_max = calculate_hybrid_delay(sleep_max, attempt, 2, 15000, 2.0, 2);
        sleep_random_range(current_min, current_max).await;
        attempt += 1;
    };
    html
}

// TODO: fetch_with_cache
