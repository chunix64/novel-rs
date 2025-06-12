use crate::db::models::PostArtist;

use super::helpers;

static TABLE_NAME: &str = "post_artists";
type Entity = PostArtist;

pub struct PostArtistRepository {
    pool: sqlx::SqlitePool,
}

impl PostArtistRepository {
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
        post_artist: &PostArtist,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!(
            "INSERT INTO {} (post_id, artist_id) VALUES (?, ?)",
            TABLE_NAME
        );
        sqlx::query(&query)
            .bind(post_artist.post_id)
            .bind(post_artist.artist_id)
            .execute(&self.pool)
            .await
    }

    pub async fn delete(
        &self,
        post_artist: &PostArtist,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!(
            "DELETE FROM {} WHERE post_id = ? AND artist_id = ?",
            TABLE_NAME
        );
        sqlx::query(&query)
            .bind(post_artist.post_id)
            .bind(post_artist.artist_id)
            .execute(&self.pool)
            .await
    }
}
