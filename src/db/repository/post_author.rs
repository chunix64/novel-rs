use crate::db::models::PostAuthor;

static TABLE_NAME: &str = "post_authors";
// type Entity = PostAuthor;

pub struct PostAuthorRepository {
    pool: sqlx::SqlitePool,
}

impl PostAuthorRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert(
        &self,
        post_author: &PostAuthor,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let query = format!(
            "INSERT INTO {} (post_id, author_id) VALUES (?, ?)",
            TABLE_NAME
        );
        sqlx::query(&query)
            .bind(post_author.post_id)
            .bind(post_author.author_id)
            .execute(&self.pool)
            .await
    }
}
