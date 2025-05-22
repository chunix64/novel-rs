use async_stream::stream;
use futures_core::Stream;
use futures_util::{StreamExt, pin_mut};

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
    pub fn get_novels(&self) -> impl Stream<Item = NovelRaw> {
        stream! {
        let html = fetch_novels(1).await;
        let max_page = parse_novel_max_page(&html);
        let novels_stream = self.get_novels_range(1, max_page);
        pin_mut!(novels_stream);
            while let Some(novel) = novels_stream.next().await {
                yield novel;
            }
        }
    }

    pub fn get_chapters_with_novel_id(
        &self,
        slug: &str,
        novel_id: i64,
    ) -> impl Stream<Item = ChapterRaw> {
        stream! {
            let html = fetch_chapters(slug).await;
            let chapter_metas = parse_chapters_list(&html);
            for (index, chapter_meta) in chapter_metas.iter().enumerate() {
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
                yield chapter_raw;
                println!("Get chapter with id {} done: {}/{}",
                    novel_id,index,
                    chapter_metas.len());
            }
        }
    }

    pub fn get_novels_range(&self, start: i64, end: i64) -> impl Stream<Item = NovelRaw> {
        stream! {
            println!("Start get Novels!");
            for i in start..=end {
            let html = fetch_novels(i).await;
            let part = parse_novels(&html);
            for novel in part {
                yield novel;
            }
            println!("Get novel done: {}/{}", i, end);
        }
            println!("Finished get Novels!");
        }
    }
}
