use crate::{
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
}

impl SiteService for NovelService {
    async fn run(&self) {
        self.sync_novels().await;
        self.sync_all_novel_chapters().await;
    }
}

impl NovelService {
    pub fn new(provider: DoclnProvider, database: Database) -> Self {
        Self { provider, database }
    }

    pub async fn sync_novels(&self) {
        let raw_novels = self.provider.get_novels().await;

        for raw_novel in raw_novels {
            let novel: Novel = raw_novel.into();
            if !self.database.novel.slug_exists(&novel.slug).await {
                self.database.novel.insert(&novel).await.unwrap();
                println!("Inserted novel: {}", &novel.id);
            }
        }
    }

    pub async fn sync_all_novel_chapters(&self) {
        let novels = self.database.novel.get_all().await.unwrap();
        for novel in novels {
            let id = novel.id;
            self.sync_chapters_for_novel(id).await;
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
        let raw_chapters = self.provider.get_chapters_with_novel_id(&slug, id).await;
        for raw_chapter in raw_chapters {
            let chapter: Chapter = raw_chapter.into();
            if !self.database.chapter.slug_exists(&chapter.slug).await {
                self.database.chapter.insert(&chapter).await.unwrap();
                println!("Inserted chapter: {}", &chapter.id);
            }
        }
    }
}
