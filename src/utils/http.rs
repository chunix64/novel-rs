use tracing::{error, trace};

pub async fn fetch_url(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    match reqwest::get(url).await {
        Ok(response) => {
            trace!(target = %"utils", %url, "Fetch url success");
            Ok(response)
        }
        Err(error) => {
            error!(target = %"utils", %url, ?error, "Failed to fetch url");
            Err(error)
        }
    }
}
