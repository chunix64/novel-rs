use tracing::warn;

use crate::{
    cache::manager::CacheManager,
    utils::{
        http::fetch_url,
        time::{calculate_hybrid_delay, sleep_random_range},
    },
};

// Main function
pub async fn fetch_novels_by_id_wrapper(
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
        fetch_novels_retry(index, sleep_min, sleep_max, max_retry).await
    }
}

pub async fn fetch_novels_wrapper(
    slug: &str,
    sleep_min: u64,
    sleep_max: u64,
    max_retry: Option<u64>,
    cache_manager: &CacheManager,
    is_cache: bool,
) -> Option<String> {
    if is_cache {
        return fetch_slug_retry_with_cache(
            slug,
            sleep_min,
            sleep_max,
            max_retry,
            "novels",
            cache_manager,
        )
        .await;
    } else {
        return fetch_slug_retry(slug, sleep_min, sleep_max, max_retry).await;
    }
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
        return fetch_slug_retry_with_cache(
            slug,
            sleep_min,
            sleep_max,
            max_retry,
            "chapters",
            cache_manager,
        )
        .await;
    } else {
        return fetch_slug_retry(slug, sleep_min, sleep_max, max_retry).await;
    }
}

// Novel function
async fn fetch_novels(index: i64) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!("https://docln.net/danh-sach?page={}", index);
    fetch_url(&url).await
}

async fn fetch_novels_retry(
    index: i64,
    sleep_min: u64,
    sleep_max: u64,
    max_retry: Option<u64>,
) -> Option<String> {
    let html: Option<String> =
        fetch_with_retry(|| fetch_novels(index), sleep_min, sleep_max, max_retry).await;
    html
}

async fn fetch_novels_retry_with_cache(
    index: i64,
    sleep_min: u64,
    sleep_max: u64,
    max_retry: Option<u64>,
    cache_manager: &CacheManager,
) -> Option<String> {
    let fetch_fn = || fetch_novels_retry(index, sleep_min, sleep_max, max_retry);
    let sub_path = "novels";
    let file_name = format!("page-{}.html", index);
    fetch_with_cache(fetch_fn, sub_path, &file_name, cache_manager).await
}

async fn fetch_slug(slug: &str) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!("https://docln.net{}", slug);
    fetch_url(&url).await
}

// slug
async fn fetch_slug_retry(
    slug: &str,
    sleep_min: u64,
    sleep_max: u64,
    max_retry: Option<u64>,
) -> Option<String> {
    let html: Option<String> =
        fetch_with_retry(|| fetch_slug(slug), sleep_min, sleep_max, max_retry).await;
    html
}

async fn fetch_slug_retry_with_cache(
    slug: &str,
    sleep_min: u64,
    sleep_max: u64,
    max_retry: Option<u64>,
    sub_path: &str,
    cache_manager: &CacheManager,
) -> Option<String> {
    let fetch_fn = || fetch_slug_retry(slug, sleep_min, sleep_max, max_retry);
    let file_name = format!("{}.html", slug);
    fetch_with_cache(fetch_fn, sub_path, &file_name, cache_manager).await
}

// helper
async fn fetch_with_retry<F, Fut>(
    mut fetch_fn: F,
    sleep_min: u64,
    sleep_max: u64,
    max_retry: Option<u64>,
) -> Option<String>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
{
    let mut attempt = 1;
    // sleep should be never <= 0 and max should be > min
    let sleep_min = sleep_min.max(2);
    let sleep_max = sleep_max.max(3);
    loop {
        if let Ok(response) = fetch_fn().await {
            let status = response.status();

            match response.text().await {
                Ok(content) if status.is_success() => {
                    return Some(content);
                }
                Ok(content) => {
                    warn!(
                        target: "provider",
                        status = %status,
                        "fetch STATUS non-200 with content, try to use"
                    );
                    break Some(content);
                }
                Err(error) => {
                    warn!(
                        target: "provider",
                        status = %status,
                        ?error,
                        "Failed to fetch data"
                    );
                    break None;
                }
            };
        };

        warn!(target = %"provider", %attempt, "Failed fetch data, retrying");

        if let Some(max) = max_retry {
            // sleep_rate is same with attempt
            if attempt > max {
                return None;
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
    }
}

async fn fetch_with_cache<F, Fut>(
    mut fetch_fn: F,
    sub_path: &str,
    file_name: &str,
    cache_manager: &CacheManager,
) -> Option<String>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Option<String>>,
{
    if cache_manager.is_exists(sub_path, file_name).await {
        return cache_manager.load(sub_path, file_name).await;
    } else {
        match fetch_fn().await {
            Some(html) => {
                cache_manager.save(sub_path, file_name, &html).await;
                Some(html)
            }
            None => None,
        }
    }
}
