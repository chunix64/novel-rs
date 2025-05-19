use crate::{
    db::{Database, models::Novel},
    site::docln::provider::DoclnProvider,
};

pub struct NovelService {
    provider: DoclnProvider,
    database: Database,
}

impl NovelService {
    pub fn new(provider: DoclnProvider, database: Database) -> Self {
        Self { provider, database }
    }

    pub async fn sync_novels(&self) {
        let raw_novels = self.provider.get_novels().await;

        for raw_novel in raw_novels {
            let novel: Novel = raw_novel.into();
            self.database.novel.insert(&novel).await.unwrap();
            println!("Inserted: {}", &novel.id);
        }
    }

    pub async fn sync_all_novel_chapters() {}

    pub async fn sync_chapters_for_novel(id: i64) {}

    pub async fn test(&self) -> Vec<crate::site::content::novels::ChapterRaw> {
        let slug = self.database.novel.get_limit(1).await.unwrap()[0]
            .slug
            .clone();
        self.provider.get_chapters(&slug).await
    }
}
