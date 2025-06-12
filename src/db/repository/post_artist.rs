use crate::db::models::PostArtist;

static TABLE_NAME: &str = "post_artists";
// type Entity = PostArtist;

pub struct PostArtistRepository {
    pool: sqlx::SqlitePool,
}

impl PostArtistRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
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
}
