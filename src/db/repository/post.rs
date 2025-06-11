use crate::db::models::Post;

use super::helpers;

static TABLE_NAME: &str = "posts";
type Entity = Post;

pub struct PostRepository {
    pool: sqlx::SqlitePool,
}

impl PostRepository {
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
        post: &Post,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!(
            "INSERT INTO {} 
            (
            content_type_id,
            title,
            slug,
            thumbnail,
            description,
            created_at,
            updated_at
            ) 
            VALUES
            (?, ?, ?, ?, ?, ?, ?)",
            TABLE_NAME
        );
        sqlx::query(&query)
            .bind(post.content_type_id)
            .bind(post.title.clone())
            .bind(post.slug.clone())
            .bind(post.thumbnail.clone())
            .bind(post.description.clone())
            .bind(post.created_at)
            .bind(post.updated_at)
            .execute(&self.pool)
            .await
    }

    pub async fn delete(&self, id: i64) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        helpers::delete(&self.pool, id, TABLE_NAME).await
    }

    pub async fn slug_exists(&self, slug: &str) -> bool {
        helpers::slug_exists(&self.pool, slug, TABLE_NAME).await
    }

    pub async fn count(&self) -> i64 {
        helpers::count(&self.pool, TABLE_NAME).await
    }
}
