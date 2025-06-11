use crate::db::models::PostTag;

use super::helpers;

static TABLE_NAME: &str = "post_tags";
type Entity = PostTag;

pub struct PostTagRepository {
    pool: sqlx::SqlitePool,
}

impl PostTagRepository {
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
        post_tag: &PostTag,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!("INSERT INTO {} (post_id, tag_id) VALUES (?, ?)", TABLE_NAME);
        sqlx::query(&query)
            .bind(post_tag.post_id)
            .bind(post_tag.tag_id)
            .execute(&self.pool)
            .await
    }

    pub async fn delete(
        &self,
        post_tag: &PostTag,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!(
            "DELETE FROM {} WHERE post_id = ? AND tag_id = ?",
            TABLE_NAME
        );
        sqlx::query(&query)
            .bind(post_tag.post_id)
            .bind(post_tag.tag_id)
            .execute(&self.pool)
            .await
    }
}
