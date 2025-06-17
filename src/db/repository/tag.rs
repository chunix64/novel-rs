use crate::db::models::Tag;

use super::helpers;

static TABLE_NAME: &str = "tags";
type Entity = Tag;

pub struct TagRepository {
    pool: sqlx::SqlitePool,
}

impl TagRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Entity, sqlx::Error> {
        helpers::get_one_by_name(&self.pool, name, TABLE_NAME).await
    }

    pub async fn get_or_insert_id(
        &self,
        name: &str,
        description: Option<&str>,
        tag_category_id: i64,
    ) -> Result<i64, sqlx::Error> {
        if !self.name_exist(name).await {
            self.insert(name, description, tag_category_id).await?;
        }
        Ok(self.get_by_name(name).await?.id)
    }

    pub async fn insert(
        &self,
        name: &str,
        description: Option<&str>,
        tag_category_id: i64,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!(
            "INSERT INTO {} (name, description ,tag_category_id) VALUES (?, ?, ?)",
            TABLE_NAME
        );
        sqlx::query(&query)
            .bind(name)
            .bind(description)
            .bind(tag_category_id)
            .execute(&self.pool)
            .await
    }

    pub async fn name_exist(&self, name: &str) -> bool {
        helpers::name_exists(&self.pool, name, TABLE_NAME).await
    }
}
