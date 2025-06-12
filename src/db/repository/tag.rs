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

    pub async fn get_all(&self) -> Result<Vec<Entity>, sqlx::Error> {
        helpers::get_all::<Entity>(&self.pool, TABLE_NAME).await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Entity, sqlx::Error> {
        helpers::get_by_id::<Entity>(&self.pool, id, TABLE_NAME).await
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Entity, sqlx::Error> {
        helpers::get_one_by_name(&self.pool, name, TABLE_NAME).await
    }

    pub async fn get_or_insert_id(&self, name: &str, tag_category_id: i64) -> i64 {
        if !self.name_exist(name).await {
            self.insert(name, tag_category_id).await.unwrap();
        }
        self.get_by_name(name).await.unwrap().id
    }

    pub async fn insert(
        &self,
        name: &str,
        tag_category_id: i64,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!(
            "INSERT INTO {} (name, tag_category_id) VALUES (?, ?)",
            TABLE_NAME
        );
        sqlx::query(&query)
            .bind(name)
            .bind(tag_category_id)
            .execute(&self.pool)
            .await
    }

    pub async fn delete(&self, id: i64) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        helpers::delete(&self.pool, id, TABLE_NAME).await
    }

    pub async fn name_exist(&self, name: &str) -> bool {
        helpers::name_exists(&self.pool, name, TABLE_NAME).await
    }
}
