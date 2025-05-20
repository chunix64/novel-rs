use crate::{
    site::{
        content::novels::{ChapterRaw, NovelRaw},
        docln::{html::fetch_novels, parser::parse_novel_max_page},
    },
    utils::time::current_stamp,
};

use super::{
    html::fetch_chapters,
    parser::{parse_chapter_content, parse_chapters_list, parse_novels},
};

pub struct DoclnProvider;

impl DoclnProvider {
    pub async fn get_novels(&self) -> Vec<NovelRaw> {
        let html = fetch_novels(1).await;
        let max_page = parse_novel_max_page(&html);
        self.get_novels_range(1, max_page).await
    }

    pub async fn get_chapters_with_novel_id(
        &self,
        slug: &String,
        novel_id: i64,
    ) -> Vec<ChapterRaw> {
        let mut result: Vec<ChapterRaw> = Vec::new();
        let html = fetch_chapters(slug).await;
        let chapter_metas = parse_chapters_list(&html);
        println!("Start get Chapter!");
        for (index, chapter_meta) in chapter_metas.iter().enumerate() {
            println!("Start get: {}", index);
            let chapter_html = fetch_chapters(&chapter_meta.slug).await;
            let content = parse_chapter_content(&chapter_html);
            let now = current_stamp() as i64;
            let chapter_raw = ChapterRaw {
                title: chapter_meta.title.clone(),
                slug: chapter_meta.slug.clone(),
                novel_id,
                created_at: now,
                updated_at: now,
                content,
                chapter_number: Some(index as i64),
            };
            result.push(chapter_raw);
            println!("Get done: {}", index);
        }

        result
    }

    pub async fn get_novels_range(&self, start: i64, end: i64) -> Vec<NovelRaw> {
        println!("Start get Novels!");
        let mut items: Vec<NovelRaw> = Vec::new();
        for i in start..=end {
            let html = fetch_novels(i).await;
            let mut part = parse_novels(&html);
            items.append(&mut part);
            println!("Novels done: {}", i);
        }
        println!("Finished get Novels!");
        items
    }
}
