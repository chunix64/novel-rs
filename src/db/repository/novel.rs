use crate::db::models::Novel;

use super::helpers;

static TABLE_NAME: &str = "novels";
type Entity = Novel;

pub struct NovelRepository {
    pool: sqlx::SqlitePool,
}

impl NovelRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Entity>, sqlx::Error> {
        helpers::get_all::<Entity>(&self.pool, TABLE_NAME).await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Entity, sqlx::Error> {
        helpers::get_by_id::<Entity>(&self.pool, id, TABLE_NAME).await
    }

    pub async fn get_limit(&self, count: i64) -> Result<Vec<Entity>, sqlx::Error> {
        helpers::get_limit(&self.pool, TABLE_NAME, count).await
    }

    pub async fn insert(
        &self,
        novel: &Novel,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!(
            "INSERT INTO {} (title, slug, thumbnail, description, author_id, artist_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            TABLE_NAME
        );
        sqlx::query(&query)
            .bind(novel.title.clone())
            .bind(novel.slug.clone())
            .bind(novel.thumbnail.clone())
            .bind(novel.description.clone())
            .bind(novel.author_id)
            .bind(novel.artist_id)
            .bind(novel.created_at)
            .bind(novel.updated_at)
            .execute(&self.pool)
            .await
    }

    pub async fn update(
        &self,
        title: &str,
        slug: &str,
        thumbnail: Option<&str>,
        description: Option<&str>,
        author_id: Option<i64>,
        artist_id: Option<i64>,
        updated_at: i64,
        id: i64,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!(
            "UPDATE {}
            SET title = ?, slug = ?, thumbnail = ?, description = ?, author_id = ?, artist_id = ?, updated_at = ?
            WHERE id = ?",
            TABLE_NAME,
        );
        sqlx::query(&query)
            .bind(title)
            .bind(slug)
            .bind(thumbnail)
            .bind(description)
            .bind(author_id)
            .bind(artist_id)
            .bind(updated_at)
            .bind(id)
            .execute(&self.pool)
            .await
    }

    pub async fn delete(&self, id: i64) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        helpers::delete(&self.pool, id, TABLE_NAME).await
    }
}
