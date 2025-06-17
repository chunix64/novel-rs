use crate::db::models::Artist;

use super::helpers;

static TABLE_NAME: &str = "artists";
type Entity = Artist;

pub struct ArtistRepository {
    pool: sqlx::SqlitePool,
}

impl ArtistRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Entity, sqlx::Error> {
        helpers::get_one_by_name(&self.pool, name, TABLE_NAME).await
    }

    pub async fn get_or_insert_id(&self, name: &str) -> Result<i64, sqlx::Error> {
        if !self.name_exist(name).await {
            self.insert(name).await?;
        }
        Ok(self.get_by_name(name).await?.id)
    }

    pub async fn insert(&self, name: &str) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!("INSERT INTO {} (name) VALUES (?)", TABLE_NAME);
        sqlx::query(&query).bind(name).execute(&self.pool).await
    }

    pub async fn name_exist(&self, name: &str) -> bool {
        helpers::name_exists(&self.pool, name, TABLE_NAME).await
    }
}
