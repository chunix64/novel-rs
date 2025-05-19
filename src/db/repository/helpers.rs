pub async fn get_all<T>(pool: &sqlx::SqlitePool, table_name: &str) -> Result<Vec<T>, sqlx::Error>
where
    T: for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
{
    let query = format!("SELECT * FROM {}", table_name);
    sqlx::query_as::<_, T>(&query).fetch_all(pool).await
}

pub async fn get_by_id<T>(
    pool: &sqlx::SqlitePool,
    id: i64,
    table_name: &str,
) -> Result<T, sqlx::Error>
where
    T: for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
{
    let query = format!("SELECT * FROM {} WHERE id = ?", table_name);
    sqlx::query_as::<_, T>(&query)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn get_limit<T>(
    pool: &sqlx::SqlitePool,
    table_name: &str,
    count: i64,
) -> Result<Vec<T>, sqlx::Error>
where
    T: for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
{
    let query = format!("SELECT * FROM {} LIMIT ?", table_name);
    sqlx::query_as::<_, T>(&query)
        .bind(count)
        .fetch_all(pool)
        .await
}

pub async fn delete(
    pool: &sqlx::SqlitePool,
    id: i64,
    table_name: &str,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let query = format!("DELETE FROM {} WHERE id = ?", table_name);
    sqlx::query(&query).bind(id).execute(pool).await
}
