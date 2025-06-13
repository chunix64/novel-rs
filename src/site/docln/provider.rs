use async_stream::stream;
use futures_core::Stream;
use futures_util::{StreamExt, pin_mut};
use tracing::info;

use crate::{
    cache::manager::CacheManager,
    config::provider::ProviderConfig,
    site::{
        content::novels::{ChapterRaw, NovelEnrich, NovelRaw},
        docln::{
            http::{fetch_chapters_wrapper, fetch_novels_wrapper},
            parser::{parse_novel_enrich, parse_novel_max_page},
        },
    },
    utils::time::sleep_random_range,
};

use super::parser::{parse_chapter, parse_chapter_content, parse_chapters_list, parse_novels};

pub struct DoclnProvider {
    config: ProviderConfig,
    cache_manager: CacheManager,
}

impl DoclnProvider {
    pub fn new(config: ProviderConfig, cache_manager: CacheManager) -> Self {
        Self {
            config,
            cache_manager,
        }
    }

    pub fn get_novels(&self) -> impl Stream<Item = NovelRaw> {
        stream! {
            let html = fetch_novels_wrapper(
                1,
                self.config.delay_min(),
                self.config.delay_max(),
                None,
                &self.cache_manager,
                self.config.is_cache(),
            )
            .await
            .unwrap();
            let max_page = parse_novel_max_page(&html);
            let novels_stream = self.get_novels_range(1, max_page);
            pin_mut!(novels_stream);
            while let Some(novel) = novels_stream.next().await {
                yield novel;
            }
        }
    }

    pub fn get_chapters_with_novel_slug(
        &self,
        slug: &str,
        novel_id: i64,
    ) -> impl Stream<Item = ChapterRaw> {
        stream! {
            let html = fetch_chapters_wrapper(
                slug,
                self.config.delay_min(),
                self.config.delay_max(),
                None,
                &self.cache_manager,
                self.config.is_cache(),
            )
            .await
            .unwrap();
            let chapter_metas = parse_chapters_list(&html);
            for (index, chapter_meta) in chapter_metas.iter().enumerate() {
                let chapter_html = fetch_chapters_wrapper(
                    &chapter_meta.slug,
                    self.config.delay_min(),
                    self.config.delay_max(),
                    None,
                    &self.cache_manager,
                    self.config.is_cache(),
                )
                .await
                .unwrap();
                let content = parse_chapter_content(&chapter_html);
                let chapter_raw = parse_chapter(chapter_meta, index as i64, novel_id, content);
                yield chapter_raw;
                info!(
                    %novel_id,
                    %index,
                    total = %chapter_metas.len(),
                    "Get chapter done",
                );
                self.sleep().await;
            }
        }
    }

    pub fn get_novels_range(&self, start: i64, end: i64) -> impl Stream<Item = NovelRaw> {
        stream! {
            info!(%start, %end, "Start get novels");
            for i in start..=end {
                let html = fetch_novels_wrapper(
                    i,
                    self.config.delay_min(),
                    self.config.delay_max(),
                    None,
                    &self.cache_manager,
                    self.config.is_cache(),
                )
                .await
                .unwrap();
                let part = parse_novels(&html);
                for novel in part {
                    yield novel;
                }
                info!(index = %i, total = %end, "Get part of novel done");
                self.sleep().await;
            }
            info!(%start, %end, "Finished get Novels");
        }
    }

    pub async fn get_novel_enrich(&self, slug: &str) -> NovelEnrich {
        let enrich_html = fetch_chapters_wrapper(
            slug,
            self.config.delay_min(),
            self.config.delay_max(),
            None,
            &self.cache_manager,
            self.config.is_cache(),
        )
        .await
        .unwrap();

        parse_novel_enrich(&enrich_html)
    }

    async fn sleep(&self) {
        sleep_random_range(self.config.delay_min(), self.config.delay_max()).await;
    }
}
