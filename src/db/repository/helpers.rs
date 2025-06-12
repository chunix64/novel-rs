#![allow(dead_code)]

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

pub async fn get_one_by_name<T>(
    pool: &sqlx::SqlitePool,
    name: &str,
    table_name: &str,
) -> Result<T, sqlx::Error>
where
    T: for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
{
    let query = format!("SELECT * FROM {} WHERE name = ?", table_name);
    sqlx::query_as(&query).bind(name).fetch_one(pool).await
}

pub async fn get_one_by_slug<T>(
    pool: &sqlx::SqlitePool,
    slug: &str,
    table_name: &str,
) -> Result<T, sqlx::Error>
where
    T: for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
{
    let query = format!("SELECT * FROM {} WHERE slug = ?", table_name);
    sqlx::query_as(&query).bind(slug).fetch_one(pool).await
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

pub async fn slug_exists(pool: &sqlx::SqlitePool, slug: &str, table_name: &str) -> bool {
    let query = format!("SELECT slug FROM {} WHERE slug = ?", table_name);
    match sqlx::query_scalar::<_, String>(&query)
        .bind(slug)
        .fetch_optional(pool)
        .await
    {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(e) => {
            println!("{:#?}", e);
            false
        }
    }
}

pub async fn name_exists(pool: &sqlx::SqlitePool, name: &str, table_name: &str) -> bool {
    let query = format!("SELECT name FROM {} WHERE name = ?", table_name);
    match sqlx::query_scalar::<_, String>(&query)
        .bind(name)
        .fetch_optional(pool)
        .await
    {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(e) => {
            println!("{:#?}", e);
            false
        }
    }
}

pub async fn count(pool: &sqlx::SqlitePool, table_name: &str) -> i64 {
    let query = format!("SELECT COUNT(*) FROM {}", table_name);
    match sqlx::query_scalar::<_, i64>(&query).fetch_one(pool).await {
        Ok(count) => count,
        Err(e) => {
            println!("{:#?}", e);
            0
        }
    }
}

pub async fn update_updated_at(
    pool: &sqlx::SqlitePool,
    table_name: &str,
    id: i64,
    updated_at: i64,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let query = format!("UPDATE {} SET updated_at = ? WHERE id = ?", table_name);
    sqlx::query(&query)
        .bind(updated_at)
        .bind(id)
        .execute(pool)
        .await
}
