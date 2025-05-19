use crate::utils::http::fetch_url;

pub async fn fetch_novels(index: i64) -> String {
    let url = format!("https://docln.net/danh-sach?page={}", index);
    fetch_url(&url).await.text().await.unwrap()
}

pub async fn fetch_chapters(slug: &String) -> String {
    let url = format!("https://docln.net{}", slug);
    fetch_url(&url).await.text().await.unwrap()
}

// TODO: fetch_with_cache
