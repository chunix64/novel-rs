use crate::{
    db::{Database, models::Novel},
    site::{docln::provider::DoclnProvider, provider_base::ContentProvider},
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
        let raw_novels = self.provider.get_items().await;

        for raw_novel in raw_novels {
            let novel: Novel = raw_novel.into();
            self.database.novel.insert(&novel).await.unwrap();
            println!("Inserted: {}", &novel.id);
        }
    }
}
