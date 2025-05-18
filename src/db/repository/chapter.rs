use crate::db::models::Chapter;

use super::helpers;

static TABLE_NAME: &str = "chapters";
type Entity = Chapter;

pub struct ChapterRepository {
    pool: sqlx::SqlitePool,
}

impl ChapterRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Entity>, sqlx::Error> {
        helpers::get_all::<Entity>(&self.pool, TABLE_NAME).await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Entity, sqlx::Error> {
        helpers::get_by_id::<Entity>(&self.pool, id, TABLE_NAME).await
    }

    pub async fn insert(
        &self,
        chapter: &Chapter,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!(
            "INSERT INTO {} (title, novel_id, created_at, updated_at, content, chapter_number) VALUES (?, ?, ?, ?, ?, ?)",
            TABLE_NAME
        );
        sqlx::query(&query)
            .bind(chapter.title.clone())
            .bind(chapter.novel_id)
            .bind(chapter.created_at)
            .bind(chapter.updated_at)
            .bind(chapter.content.clone())
            .bind(chapter.chapter_number)
            .execute(&self.pool)
            .await
    }

    pub async fn update(
        &self,
        title: &str,
        novel_id: Option<i64>,
        updated_at: i64,
        content: &str,
        chapter_number: Option<i64>,
        id: i64,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!(
            "UPDATE {}
            SET title = ?, novel_id = ?, updated_at = ?, content = ?, chapter_number = ?
            WHERE id = ?",
            TABLE_NAME,
        );
        sqlx::query(&query)
            .bind(title)
            .bind(novel_id)
            .bind(updated_at)
            .bind(content)
            .bind(chapter_number)
            .bind(id)
            .execute(&self.pool)
            .await
    }

    pub async fn delete(&self, id: i64) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        helpers::delete(&self.pool, id, TABLE_NAME).await
    }
}
