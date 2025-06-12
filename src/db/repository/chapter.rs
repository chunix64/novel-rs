use crate::db::models::Chapter;

use super::helpers;

static TABLE_NAME: &str = "chapters";
// type Entity = Chapter;

pub struct ChapterRepository {
    pool: sqlx::SqlitePool,
}

impl ChapterRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert(
        &self,
        chapter: &Chapter,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!(
            "INSERT INTO {}
            (title, slug, post_id, created_at, updated_at, content, chapter_number)
            VALUES
            (?, ?, ?, ?, ?, ?, ?)",
            TABLE_NAME
        );
        sqlx::query(&query)
            .bind(chapter.title.clone())
            .bind(chapter.slug.clone())
            .bind(chapter.post_id)
            .bind(chapter.created_at)
            .bind(chapter.updated_at)
            .bind(chapter.content.clone())
            .bind(chapter.chapter_number)
            .execute(&self.pool)
            .await
    }

    pub async fn slug_exists(&self, slug: &str) -> bool {
        helpers::slug_exists(&self.pool, slug, TABLE_NAME).await
    }
}
