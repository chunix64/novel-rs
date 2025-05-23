use futures_util::{StreamExt, pin_mut};

use crate::{
    config::app::AppConfig,
    db::{
        Database,
        models::{Chapter, Novel},
    },
    site::docln::provider::DoclnProvider,
};

use super::SiteService;

pub struct NovelService {
    provider: DoclnProvider,
    database: Database,
    app_config: AppConfig,
}

impl SiteService for NovelService {
    async fn run(&self) {}
}

impl NovelService {
    pub fn new(provider: DoclnProvider, database: Database, app_config: AppConfig) -> Self {
        Self {
            provider,
            database,
            app_config,
        }
    }

    pub async fn sync_novels(&self) {
        let raw_novels = self.provider.get_novels();

        pin_mut!(raw_novels);
        while let Some(raw_novel) = raw_novels.next().await {
            let novel: Novel = raw_novel.into();
            if !self.database.novel.slug_exists(&novel.slug).await {
                self.database.novel.insert(&novel).await.unwrap();
                println!("Count: {}", self.database.novel.count().await);
            } else {
                println!("Skip get novels for {}: {}", novel.id, novel.title);
            }
        }
    }

    pub async fn sync_all_novel_chapters(&self) {
        let novels = self.database.novel.get_all().await.unwrap();
        for (index, novel) in novels.iter().enumerate() {
            let id = novel.id;
            if !self.database.chapter.slug_exists(&novel.slug).await {
                println!("Start get chapters for {}/{}", index, novels.len());
                self.sync_chapters_for_novel(id).await;
            } else {
                println!("Skip get chapters for {}/{}", index, novels.len());
            }
        }
    }

    pub async fn sync_chapters_for_novel(&self, id: i64) {
        let slug = self
            .database
            .novel
            .get_by_id(id)
            .await
            .unwrap()
            .slug
            .clone();
        let raw_chapters = self.provider.get_chapters_with_novel_id(&slug, id);
        pin_mut!(raw_chapters);
        while let Some(raw_chapter) = raw_chapters.next().await {
            let chapter: Chapter = raw_chapter.into();
            if !self.database.chapter.slug_exists(&chapter.slug).await {
                self.database.chapter.insert(&chapter).await.unwrap();
                println!("Inserted chapter: {}", &chapter.id);
            }
        }
    }

    pub async fn test(&self) {
        // println!("test: {:#?}", self.database.novel.slug_exists(&novel.slug).await);
    }
}
