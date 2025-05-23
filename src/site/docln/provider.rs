use async_stream::stream;
use futures_core::Stream;
use futures_util::{StreamExt, pin_mut};

use crate::{
    config::provider::ProviderConfig,
    site::{
        content::novels::{ChapterRaw, NovelRaw},
        docln::{
            html::{fetch_chapters_retry, fetch_novels_retry},
            parser::parse_novel_max_page,
        },
    },
    utils::time::{current_stamp, sleep_random_range},
};

use super::parser::{parse_chapter_content, parse_chapters_list, parse_novels};

pub struct DoclnProvider {
    config: ProviderConfig,
}

impl DoclnProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self { config }
    }

    pub fn get_novels(&self) -> impl Stream<Item = NovelRaw> {
        stream! {
        let html = fetch_novels_retry(1, self.config.delay_min(),
        self.config.delay_max(), None)
            .await.unwrap();
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
            let html = fetch_chapters_retry(slug,
                    self.config.delay_min(),
                    self.config.delay_max(),
                    None
                ).await.unwrap();
            let chapter_metas = parse_chapters_list(&html);
            for (index, chapter_meta) in chapter_metas.iter().enumerate() {
                let chapter_html = fetch_chapters_retry(&chapter_meta.slug,
                    self.config.delay_min(),
                    self.config.delay_max(),
                    None
                    ).await.unwrap();
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
                self.sleep().await;
            }
        }
    }

    pub fn get_novels_range(&self, start: i64, end: i64) -> impl Stream<Item = NovelRaw> {
        stream! {
            println!("Start get Novels!");
            for i in start..=end {
                let html = fetch_novels_retry(i, self.config.delay_min(),
                    self.config.delay_max(), None)
                    .await.unwrap();
                std::fs::write(format!("data/cache/debug-page-{}.html", i), &html).expect("Failed Write");
                let part = parse_novels(&html);
                for novel in part {
                    yield novel;
                }
                println!("Get novel done: {}/{}", i, end);
                self.sleep().await;
            }
            println!("Finished get Novels!");
        }
    }

    async fn sleep(&self) {
        sleep_random_range(self.config.delay_min(), self.config.delay_max()).await;
    }
}
