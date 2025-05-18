#![allow(dead_code)]

use sqlx::FromRow;

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
}

#[derive(FromRow, Debug)]
pub struct Novel {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub thumbnail: Option<String>,
    pub description: Option<String>,
    pub author_id: Option<i64>,
    pub artist_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(FromRow, Debug)]
pub struct NovelTag {
    pub novel_id: i64,
    pub tag_id: i64,
}

#[derive(FromRow, Debug)]
pub struct Chapter {
    pub id: i64,
    pub title: String,
    pub novel_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub content: String,
    pub chapter_number: Option<i64>,
}
