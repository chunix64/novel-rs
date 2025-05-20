use sqlx::SqlitePool;

pub async fn init_db(db: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("PRAGMA foreign_keys = ON;").execute(db).await?;
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS authors (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );"#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS artists (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );"#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tags (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL
    );"#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS novels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            slug TEXT NOT NULL,
            thumbnail TEXT,
            author_id INTEGER,
            artist_id INTEGER,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            description TEXT,
            FOREIGN KEY(author_id) REFERENCES authors(id) ON DELETE SET NULL,
            FOREIGN KEY(artist_id) REFERENCES artists(id) ON DELETE SET NULL,
            UNIQUE(slug)
        );"#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS novel_tags (
        novel_id INTEGER NOT NULL,
        tag_id INTEGER NOT NULL,
        FOREIGN KEY(novel_id) REFERENCES novels(id) ON DELETE CASCADE,
        FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE,
        PRIMARY KEY(novel_id, tag_id)
    );"#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS chapters (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            slug TEXT NOT NULL,
            novel_id INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            content TEXT NOT NULL,
            chapter_number INTEGER,
            FOREIGN KEY(novel_id) REFERENCES novels(id) ON DELETE CASCADE,
            UNIQUE(slug, novel_id)
        );"#,
    )
    .execute(db)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_novels_author_id ON novels(author_id);")
        .execute(db)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_novels_artist_id ON novels(artist_id);")
        .execute(db)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_chapters_novel_id ON chapters(novel_id);")
        .execute(db)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_novel_tags_tag_id ON novel_tags(tag_id);")
        .execute(db)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_chapters_slug_novel ON chapters(slug, novel_id);")
        .execute(db)
        .await?;

    Ok(())
}
