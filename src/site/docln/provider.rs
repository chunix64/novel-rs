use crate::site::{
    content::novels::{ChapterRaw, NovelRaw},
    docln::{html::fetch, parser::parse_max_page},
    provider_base::ContentProvider,
};

use super::parser::parse_items;

pub struct DoclnProvider;

impl ContentProvider for DoclnProvider {
    type Item = NovelRaw;
    type Content = ChapterRaw;

    async fn get_items(&self) -> Vec<Self::Item> {
        let mut items: Vec<Self::Item> = Vec::new();
        let html = fetch(1).await;
        let max_page = parse_max_page(&html);

        for i in 1..=max_page {
            let html = fetch(i).await;
            let mut part = parse_items(&html);
            items.append(&mut part);
        }

        items
    }

    async fn get_contents(&self) -> Vec<Self::Content> {
        let contents: Vec<Self::Content> = Vec::new();
        contents
    }
}
