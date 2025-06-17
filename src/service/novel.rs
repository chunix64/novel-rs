use futures_util::{StreamExt, pin_mut};
use tracing::{debug, error, info};

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
            let novel: Post = novel_raw_to_post(raw_novel, content_type_id);
            if !self.database.post.slug_exists(&novel.slug).await {
                match self.database.post.insert(&novel).await {
                    Ok(_) => info!(target = %"service", %novel.id, %novel.title, "Inserted novel"),
                    Err(error) => {
                        error!(target = %"service", %novel.id, %novel.title, ?error, "Failed to insert novel")
                    }
                };
                self.enrich_novel(&novel.slug).await;
                info!(target = %"service", %novel.id, %novel.title, "Enriched novel");
                let count = self.database.post.count().await;
                debug!(target = %"service", %count, "Current number of posts");
            } else {
                info!(target = %"service", %novel.id, %novel.title, "Skip get novel (already exists)");
            }
        }
    }

    pub async fn sync_all_novel_chapters(&self) {
        let novels = match self.database.post.get_all().await {
            Ok(novels) => {
                debug!(target = %"service", count = %novels.len(), "Got all post");
                novels
            }
            Err(error) => {
                error!(target = %"service", ?error, "Failed to get all post, skip");
                return;
            }
        };
        for (index, novel) in novels.iter().enumerate() {
            let id = novel.id;
            if !self.database.chapter.slug_exists(&novel.slug).await {
                info!(target = %"service", %index, total = %novels.len(),"Start get chapters");
                self.sync_chapters_for_novel(id).await;
            } else {
                info!(target = %"service", %index, total = %novels.len(), "Skip get chapters (already exists)");
            }
        }
    }

    pub async fn sync_chapters_for_novel(&self, novel_id: i64) {
        let post = match self.database.post.get_by_id(novel_id).await {
            Ok(post) => {
                debug!(target = %"service", post_id = %post.id, "Got post");
                post
            }
            Err(error) => {
                error!(target = %"service", ?error, "Failed to get post, skip");
                return;
            }
        };
        let slug = &post.slug;
        let raw_chapters = self.provider.get_chapters_with_novel_slug(slug, novel_id);
        pin_mut!(raw_chapters);
        while let Some(raw_chapter) = raw_chapters.next().await {
            let chapter: Chapter = raw_chapter.into();
            if !self.database.chapter.slug_exists(&chapter.slug).await {
                match self.database.chapter.insert(&chapter).await {
                    Ok(_) => info!(target = %"service", %chapter.id, %novel_id, "Inserted chapter"),
                    Err(error) => {
                        error!(target = %"service", %chapter.id, %novel_id, ?error, "Failed to inserted chapter")
                    }
                };
            }
        }
    }

    pub async fn enrich_novel(&self, slug: &str) {
        let post = match self.database.post.get_by_slug(slug).await {
            Ok(post) => {
                debug!(target = %"service", post_id = %post.id, "Got post");
                post
            }
            Err(error) => {
                error!(target = %"service", ?error, "Failed to get post, skip");
                return;
            }
        };
        let post_id = post.id;
        let enrich_novel = match self.provider.get_novel_enrich(slug).await {
            Some(enrich_novel) => enrich_novel,
            None => {
                error!(target = %"service", %post_id, "Failed to get novel enrich");
                return;
            }
        };

        if let Some(description) = enrich_novel.description {
            match self
                .database
                .post
                .update_description(post_id, &description)
                .await
            {
                Ok(_) => debug!(target = %"service", %post.id, "Updated description"),
                Err(error) => {
                    error!(target = %"service", %post.id, ?error, "Failed to update description")
                }
            };
        }
        match self
            .database
            .post
            .update_updated_at(post_id, enrich_novel.updated_at)
            .await
        {
            Ok(_) => debug!(target = %"service", %post.id, "Updated created_at"),
            Err(error) => {
                error!(target = %"service", %post.id, ?error, "Failed to update created_at")
            }
        };

        match self.database.post.update_is_enriched(post_id, true).await {
            Ok(_) => debug!(target = %"service", %post.id, "Updated is_enriched"),
            Err(error) => {
                error!(target = %"service", %post.id, ?error, "Failed to update is_enriched")
            }
        };

        for author in enrich_novel.authors {
            let author_id = match self.database.author.get_or_insert_id(&author).await {
                Ok(author_id) => author_id,
                Err(error) => {
                    error!(target = %"service", %author, ?error, "Failed to get author_id");
                    continue;
                }
            };
            let post_author = PostAuthor { post_id, author_id };
            match self.database.post_author.insert(&post_author).await {
                Ok(_) => debug!(target = %"service", %post.id, %author_id, "Updated author"),
                Err(error) => {
                    error!(target = %"service", %post.id, %author_id, ?error, "Failed to update author")
                }
            };
        }

        for artist in enrich_novel.artists {
            let artist_id = match self.database.artist.get_or_insert_id(&artist).await {
                Ok(artist_id) => artist_id,
                Err(error) => {
                    error!(target = %"service", %artist, ?error, "Failed to get artist_id");
                    continue;
                }
            };
            let post_artist = PostArtist { post_id, artist_id };
            match self.database.post_artist.insert(&post_artist).await {
                Ok(_) => debug!(target = %"service", %post.id, %artist_id, "Updated artist"),
                Err(error) => {
                    error!(target = %"service", %post.id, %artist_id, ?error, "Failed to artist")
                }
            };
        }

        for tag in enrich_novel.tags {
            let tag_category_id = 1;
            let tag_id = match self
                .database
                .tag
                .get_or_insert_id(&tag, None, tag_category_id)
                .await
            {
                Ok(tag_id) => tag_id,
                Err(error) => {
                    error!(target = %"service", %tag, ?error, "Failed to get tag_id");
                    continue;
                }
            };
            let post_tag = PostTag { post_id, tag_id };

            match self.database.post_tag.insert(&post_tag).await {
                Ok(_) => debug!(target = %"service", %post.id, %tag_id, "Updated tag"),
                Err(error) => {
                    error!(target = %"service", %post.id, %tag_id, ?error, "Failed to tag")
                }
            };
        }
    }
}
