use super::content::base::{ContentRaw, ItemRaw};

pub trait ContentProvider {
    type Item: ItemRaw + Send + Sync;
    type Content: ContentRaw + Send + Sync;

    async fn get_items(&self) -> Vec<Self::Item>;
    async fn get_contents(&self) -> Vec<Self::Content>;
}
