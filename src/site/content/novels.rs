use super::base::{ContentRaw, ItemRaw};

#[derive(Debug)]
pub struct NovelRaw {
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

#[derive(Debug)]
pub struct ChapterRaw {
    pub title: String,
    pub slug: String,
    pub novel_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub content: String,
    pub chapter_number: Option<i64>,
}

#[derive(Debug)]
pub struct ChapterMeta {
    pub title: String,
    pub slug: String,
}

impl ItemRaw for NovelRaw {}
impl ContentRaw for ChapterRaw {}
