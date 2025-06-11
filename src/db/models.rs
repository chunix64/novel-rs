#![allow(dead_code)]

use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct ContentType {
    pub id: i64,
    pub name: String,
}

#[derive(FromRow, Debug)]
// Post name for scale to multi-media type in future
pub struct Post {
    pub id: i64,
    pub content_type_id: i64,
    pub title: String,
    pub slug: String,
    pub thumbnail: Option<String>,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(FromRow, Debug)]
pub struct Author {
    pub id: i64,
    pub name: String,
}

#[derive(FromRow, Debug)]
pub struct Artist {
    pub id: i64,
    pub name: String,
}

#[derive(FromRow, Debug)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub category: Option<String>,
}

#[derive(FromRow, Debug)]
pub struct Chapter {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub post_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub content: String,
    pub chapter_number: Option<i64>,
}

#[derive(FromRow, Debug)]
pub struct PostTag {
    pub post_id: i64,
    pub tag_id: i64,
}

#[derive(FromRow, Debug)]
pub struct PostAuthor {
    pub post_id: i64,
    pub author_id: i64,
}

#[derive(FromRow, Debug)]
pub struct PostArtist {
    pub post_id: i64,
    pub artist_id: i64,
}
