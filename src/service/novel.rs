use futures_util::{StreamExt, pin_mut};
use tracing::{debug, info};

use crate::{
    db::{
        Database,
        models::{Chapter, Post, PostArtist, PostAuthor, PostTag},
    },
    service::convert::novel_raw_to_post,
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
        let raw_novels = self.provider.get_novels();

        pin_mut!(raw_novels);
        while let Some(raw_novel) = raw_novels.next().await {
            let content_type_id = 1;
            let novel: Post = novel_raw_to_post(&raw_novel, content_type_id);
            if !self.database.post.slug_exists(&novel.slug).await {
                self.database.post.insert(&novel).await.unwrap();
                info!(%novel.id, %novel.title, "Inserted novel");
                self.enrich_novel(&raw_novel.slug).await;
                info!(%novel.id, %novel.title, "Enriched novel");
                let count = self.database.post.count().await;
                debug!(%count, "Current number of posts");
            } else {
                info!(%novel.id, %novel.title, "Skip get novel (already exists)");
            }
        }
    }

    pub async fn sync_all_novel_chapters(&self) {
        let novels = self.database.post.get_all().await.unwrap();
        for (index, novel) in novels.iter().enumerate() {
            let id = novel.id;
            if !self.database.chapter.slug_exists(&novel.slug).await {
                info!(%index, total = %novels.len(),"Start get chapters");
                self.sync_chapters_for_novel(id).await;
            } else {
                info!(%index, total = %novels.len(), "Skip get chapters (already exists)");
            }
        }
    }

    pub async fn sync_chapters_for_novel(&self, novel_id: i64) {
        let slug = self
            .database
            .post
            .get_by_id(novel_id)
            .await
            .unwrap()
            .slug
            .clone();
        let raw_chapters = self.provider.get_chapters_with_novel_slug(&slug, novel_id);
        pin_mut!(raw_chapters);
        while let Some(raw_chapter) = raw_chapters.next().await {
            let chapter: Chapter = raw_chapter.into();
            if !self.database.chapter.slug_exists(&chapter.slug).await {
                self.database.chapter.insert(&chapter).await.unwrap();
                info!(
                    %chapter.id, %novel_id, "Inserted chapter"
                );
            }
        }
    }

    pub async fn enrich_novel(&self, slug: &str) {
        let post_id = self.database.post.get_by_slug(slug).await.unwrap().id;
        let enrich_novel = self.provider.get_novel_enrich(slug).await;

        if let Some(description) = enrich_novel.description {
            self.database
                .post
                .update_description(post_id, &description)
                .await
                .unwrap();
        }
        self.database
            .post
            .update_updated_at(post_id, enrich_novel.updated_at)
            .await
            .unwrap();
        self.database
            .post
            .update_is_enriched(post_id, true)
            .await
            .unwrap();

        for author in enrich_novel.authors {
            let author_id = self.database.author.get_or_insert_id(&author).await;
            let post_author = PostAuthor { post_id, author_id };
            self.database
                .post_author
                .insert(&post_author)
                .await
                .unwrap();
        }

        for artist in enrich_novel.artists {
            let artist_id = self.database.artist.get_or_insert_id(&artist).await;
            let post_artist = PostArtist { post_id, artist_id };
            self.database
                .post_artist
                .insert(&post_artist)
                .await
                .unwrap();
        }

        for tag in enrich_novel.tags {
            let tag_category_id = 1;
            let tag_id = self
                .database
                .tag
                .get_or_insert_id(&tag, None, tag_category_id)
                .await;
            let post_tag = PostTag { post_id, tag_id };

            self.database.post_tag.insert(&post_tag).await.unwrap();
        }
    }
}
