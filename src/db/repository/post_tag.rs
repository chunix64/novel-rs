use crate::db::models::PostTag;

static TABLE_NAME: &str = "post_tags";
// type Entity = PostTag;

pub struct PostTagRepository {
    pool: sqlx::SqlitePool,
}

impl PostTagRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
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
}
