use crate::db::models::NovelTag;

use super::helpers;

static TABLE_NAME: &str = "novel_tags";
type Entity = NovelTag;

pub struct NovelTagRepository {
    pool: sqlx::SqlitePool,
}

impl NovelTagRepository {
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
        novel_tag: &NovelTag,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!(
            "INSERT INTO {} (novel_id, tag_id) VALUES (?, ?)",
            TABLE_NAME
        );
        sqlx::query(&query)
            .bind(novel_tag.novel_id)
            .bind(novel_tag.tag_id)
            .execute(&self.pool)
            .await
    }

    pub async fn update(
        &self,
        novel_tag: &NovelTag,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!("UPDATE {} SET tag_id = ? WHERE novel_id = ?", TABLE_NAME,);
        sqlx::query(&query)
            .bind(novel_tag.tag_id)
            .bind(novel_tag.novel_id)
            .execute(&self.pool)
            .await
    }

    pub async fn delete(
        &self,
        novel_tag: &NovelTag,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!(
            "DELETE FROM {} WHERE novel_id = ? AND tag_id = ?",
            TABLE_NAME
        );
        sqlx::query(&query)
            .bind(novel_tag.novel_id)
            .bind(novel_tag.tag_id)
            .execute(&self.pool)
            .await
    }
}
