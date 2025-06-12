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

    pub async fn get_by_slug(&self, slug: &str) -> Result<Entity, sqlx::Error> {
        helpers::get_one_by_slug(&self.pool, slug, TABLE_NAME).await
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
            updated_at,
            is_enriched
            ) 
            VALUES
            (?, ?, ?, ?, ?, ?, ?, ?)",
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
            .bind(false)
            .execute(&self.pool)
            .await
    }

    pub async fn slug_exists(&self, slug: &str) -> bool {
        helpers::slug_exists(&self.pool, slug, TABLE_NAME).await
    }

    pub async fn count(&self) -> i64 {
        helpers::count(&self.pool, TABLE_NAME).await
    }

    // unusual function
    pub async fn update_description(
        &self,
        id: i64,
        description: &str,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!("UPDATE {} SET description = ? WHERE id = ?", TABLE_NAME);
        sqlx::query(&query)
            .bind(description)
            .bind(id)
            .execute(&self.pool)
            .await
    }

    pub async fn update_updated_at(
        &self,
        id: i64,
        updated_at: i64,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        helpers::update_updated_at(&self.pool, TABLE_NAME, id, updated_at).await
    }

    pub async fn update_is_enriched(
        &self,
        id: i64,
        is_enriched: bool,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!("UPDATE {} SET is_enriched = ? WHERE id = ?", TABLE_NAME);
        sqlx::query(&query)
            .bind(is_enriched)
            .bind(id)
            .execute(&self.pool)
            .await
    }
}
