use crate::{
    cache::manager::CacheManager,
    utils::{
        http::fetch_url,
        time::{calculate_hybrid_delay, sleep_random_range},
    },
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

pub async fn fetch_novels_retry_with_cache(
    index: i64,
    sleep_min: u64,
    sleep_max: u64,
    max_retry: Option<u64>,
    cache_manager: &CacheManager,
) -> Option<String> {
    let fetch_fn = || fetch_novels_retry(index, sleep_min, sleep_max, max_retry);
    let sub_path = "novels";
    let file_name = format!("page-{}.html", index.to_string());
    let html = fetch_with_cache(fetch_fn, sub_path, &file_name, cache_manager).await;
    html
}

pub async fn fetch_novels_wrapper(
    index: i64,
    sleep_min: u64,
    sleep_max: u64,
    max_retry: Option<u64>,
    cache_manager: &CacheManager,
    is_cache: bool,
) -> Option<String> {
    if is_cache {
        return fetch_novels_retry_with_cache(
            index,
            sleep_min,
            sleep_max,
            max_retry,
            cache_manager,
        )
        .await;
    } else {
        return fetch_novels_retry(index, sleep_min, sleep_max, max_retry).await;
    }
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

pub async fn fetch_chapters_retry_with_cache(
    slug: &str,
    sleep_min: u64,
    sleep_max: u64,
    max_retry: Option<u64>,
    cache_manager: &CacheManager,
) -> Option<String> {
    let fetch_fn = || fetch_chapters_retry(slug, sleep_min, sleep_max, max_retry);
    let sub_path = "chapters";
    let file_name = format!("{}.html", slug);
    let html = fetch_with_cache(fetch_fn, sub_path, &file_name, cache_manager).await;
    html
}

pub async fn fetch_chapters_wrapper(
    slug: &str,
    sleep_min: u64,
    sleep_max: u64,
    max_retry: Option<u64>,
    cache_manager: &CacheManager,
    is_cache: bool,
) -> Option<String> {
    if is_cache {
        return fetch_chapters_retry_with_cache(
            slug,
            sleep_min,
            sleep_max,
            max_retry,
            cache_manager,
        )
        .await;
    } else {
        return fetch_chapters_retry(slug, sleep_min, sleep_max, max_retry).await;
    }
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
    let sleep_min = sleep_min.max(2);
    let sleep_max = sleep_max.max(3);
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

pub async fn fetch_with_cache<F, Fut>(
    mut fetch_fn: F,
    sub_path: &str,
    file_name: &str,
    cache_manager: &CacheManager,
) -> Option<String>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Option<String>>,
{
    if cache_manager.is_exists(sub_path, &file_name).await {
        return cache_manager.load(sub_path, &file_name).await;
    } else {
        let html = fetch_fn().await.unwrap();
        cache_manager.save(sub_path, &file_name, &html).await;
        return Some(html.to_string());
    }
}
// TODO: fetch_with_cache
