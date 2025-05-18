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
        let html = fetch(1).await;
        let max_page = parse_max_page(&html);
        self.get_items_range(1, max_page).await
    }

    async fn get_contents(&self) -> Vec<Self::Content> {
        let contents: Vec<Self::Content> = Vec::new();
        contents
    }
}

impl DoclnProvider {
    pub async fn get_items_range(&self, start: i64, end: i64) -> Vec<NovelRaw> {
        println!("Start get Novels!");
        let mut items: Vec<NovelRaw> = Vec::new();
        for i in start..=end {
            let html = fetch(i).await;
            let mut part = parse_items(&html);
            items.append(&mut part);
            println!("Novels done: {}", i);
        }
        println!("Finished get Novels!");
        items
    }
}
