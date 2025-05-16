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

    pub async fn insert(
        &self,
        title: &str,
        description: Option<&str>,
        author_id: Option<i64>,
        artist_id: Option<i64>,
        created_at: i64,
        updated_at: i64,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!(
            "INSERT INTO {} (title, description, author_id, artist_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
            TABLE_NAME
        );
        sqlx::query(&query)
            .bind(title)
            .bind(description)
            .bind(author_id)
            .bind(artist_id)
            .bind(created_at)
            .bind(updated_at)
            .execute(&self.pool)
            .await
    }

    pub async fn update(
        &self,
        title: &str,
        description: Option<&str>,
        author_id: Option<i64>,
        artist_id: Option<i64>,
        updated_at: i64,
        id: i64,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!(
            "UPDATE {}
            SET title = ?, description = ?, author_id = ?, artist_id = ?, updated_at = ?
            WHERE id = ?",
            TABLE_NAME,
        );
        sqlx::query(&query)
            .bind(title)
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
