use crate::site::{
    content::novels::{ChapterRaw, NovelRaw},
    provider_base::ContentProvider,
};

use super::parser::parse_test;

pub struct DoclnProvider;

impl ContentProvider for DoclnProvider {
    type Item = NovelRaw;
    type Content = ChapterRaw;

    async fn get_items(&self) -> Vec<Self::Item> {
        let mut items: Vec<Self::Item> = Vec::new();
        for i in 0..5 {
            let mut part = parse_test(i).await;
            items.append(&mut part);
        }
        items
    }

    async fn get_contents(&self) -> Vec<Self::Content> {
        let contents: Vec<Self::Content> = Vec::new();
        contents
    }
}
