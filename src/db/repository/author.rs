use crate::db::models::Author;

use super::helpers;

static TABLE_NAME: &str = "authors";
type Entity = Author;

pub struct AuthorRepository {
    pool: sqlx::SqlitePool,
}

impl AuthorRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Entity, sqlx::Error> {
        helpers::get_one_by_name(&self.pool, name, TABLE_NAME).await
    }

    pub async fn get_or_insert_id(&self, name: &str) -> i64 {
        if !self.name_exist(name).await {
            self.insert(name).await.unwrap();
        }
        self.get_by_name(name).await.unwrap().id
    }

    pub async fn insert(&self, name: &str) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!("INSERT OR IGNORE INTO {} (name) VALUES (?)", TABLE_NAME);
        sqlx::query(&query).bind(name).execute(&self.pool).await
    }

    pub async fn name_exist(&self, name: &str) -> bool {
        helpers::name_exists(&self.pool, name, TABLE_NAME).await
    }
}
