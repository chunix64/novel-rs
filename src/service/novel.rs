use crate::{
    db::{
        Database,
        models::{Chapter, Novel},
    },
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

    pub async fn sync_all_novel_chapters(&self) {}

    pub async fn sync_chapters_for_novel(&self, id: i64) {
        let slug = self.database.novel.get_limit(1).await.unwrap()[0]
            .slug
            .clone();
        let raw_chapters = self.provider.get_chapters_with_novel_id(&slug, id).await;
        for raw_chapter in raw_chapters {
            let chapter: Chapter = raw_chapter.into();
            self.database.chapter.insert(&chapter).await.unwrap();
            println!("Inserted: {}", &chapter.id);
        }
    }

    pub async fn test(&self) {
        let id = self.database.novel.get_limit(1).await.unwrap()[0]
            .id
            .clone();
        self.sync_chapters_for_novel(id).await;
    }
}
