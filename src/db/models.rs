#![allow(dead_code)]

use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Author {
    id: i64,
    name: String,
}

#[derive(FromRow, Debug)]
pub struct Artist {
    id: i64,
    name: String,
}

#[derive(FromRow, Debug)]
pub struct Tag {
    id: i64,
    name: String,
}

#[derive(FromRow, Debug)]
pub struct Novel {
    id: i64,
    title: String,
    thumbnail: Option<String>,
    description: Option<String>,
    author_id: Option<i64>,
    artist_id: Option<i64>,
    created_at: i64,
    updated_at: i64,
}

#[derive(FromRow, Debug)]
pub struct NovelTag {
    novel_id: i64,
    tag_id: i64,
}

#[derive(FromRow, Debug)]
pub struct Chapter {
    id: i64,
    title: String,
    novel_id: i64,
    created_at: i64,
    updated_at: i64,
    content: String,
    chapter_number: Option<i64>,
}
