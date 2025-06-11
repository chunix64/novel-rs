use sqlx::SqlitePool;

pub async fn init_db(db: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("PRAGMA foreign_keys = ON;").execute(db).await?;

    init_schema(db).await;
    init_index(db).await;
    init_db_value(db).await;
    Ok(())
}

async fn init_schema(db: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS content_types (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        UNIQUE(name)
    );"#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content_type_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            slug TEXT NOT NULL,
            thumbnail TEXT,
            description TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY(content_type_id) REFERENCES content_types(id) ON DELETE RESTRICT,
            UNIQUE(slug)
        );"#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS authors (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            UNIQUE(name)
        );"#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS artists (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            UNIQUE(name)
        );"#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tags (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        category TEXT,
        UNIQUE(name)
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
            post_id INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            content TEXT NOT NULL,
            chapter_number INTEGER,
            FOREIGN KEY(post_id) REFERENCES posts(id) ON DELETE CASCADE,
            UNIQUE(slug, post_id)
        );"#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS post_tags (
        post_id INTEGER NOT NULL,
        tag_id INTEGER NOT NULL,
        FOREIGN KEY(post_id) REFERENCES posts(id) ON DELETE CASCADE,
        FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE,
        PRIMARY KEY(post_id, tag_id)
    );"#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS post_authors (
        post_id INTEGER NOT NULL,
        author_id INTEGER NOT NULL,
        FOREIGN KEY(post_id) REFERENCES posts(id) ON DELETE CASCADE,
        FOREIGN KEY(author_id) REFERENCES authors(id) ON DELETE CASCADE,
        PRIMARY KEY(post_id, author_id)
    );"#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS post_artists (
        post_id INTEGER NOT NULL,
        artist_id INTEGER NOT NULL,
        FOREIGN KEY(post_id) REFERENCES posts(id) ON DELETE CASCADE,
        FOREIGN KEY(artist_id) REFERENCES artists(id) ON DELETE CASCADE,
        PRIMARY KEY(post_id, artist_id)
    );"#,
    )
    .execute(db)
    .await?;

    Ok(())
}

// Index Optimize (not necessary, can delete)
async fn init_index(db: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_posts_content_type_id ON posts(content_type_id);")
        .execute(db)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at);")
        .execute(db)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_posts_updated_at ON posts(updated_at);")
        .execute(db)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tags_category ON tags(category);")
        .execute(db)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_chapters_slug_post_id ON chapters(slug, post_id);")
        .execute(db)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_chapters_post_id ON chapters(post_id);")
        .execute(db)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_post_tags_post_id ON post_tags(post_id);")
        .execute(db)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_post_tags_tag_id ON post_tags(tag_id);")
        .execute(db)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_post_authors_post_id ON post_authors(post_id);")
        .execute(db)
        .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_post_authors_author_id ON post_authors(author_id);",
    )
    .execute(db)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_post_artists_post_id ON post_artists(post_id);")
        .execute(db)
        .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_post_artists_artist_id ON post_artists(artist_id);",
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        CREATE VIRTUAL TABLE posts_fts USING fts5(
        title, 
        description,
        content='posts',
        tokenize = 'unicode61 remove_diacritics 2',
        prefix = '2 3 4',
        );"#,
    )
    .execute(db)
    .await?;

    Ok(())
}

async fn init_db_value(db: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO content_types (name) 
        VALUES 
        ('novel');
        "#,
    )
    .execute(db)
    .await?;

    Ok(())
}
