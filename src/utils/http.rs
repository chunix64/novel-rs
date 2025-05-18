pub async fn fetch_url(url: &str) -> reqwest::Response {
    reqwest::get(url).await.unwrap()
}
