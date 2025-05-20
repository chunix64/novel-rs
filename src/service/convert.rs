use crate::{
    db::models::{Chapter, Novel},
    site::content::novels::{ChapterRaw, NovelRaw},
};

impl From<NovelRaw> for Novel {
    fn from(raw: NovelRaw) -> Self {
        Self {
            id: raw.id,
            title: raw.title,
            slug: raw.slug,
            thumbnail: raw.thumbnail,
            description: raw.description,
            author_id: raw.author_id,
            artist_id: raw.artist_id,
            created_at: raw.created_at,
            updated_at: raw.updated_at,
        }
    }
}

impl From<ChapterRaw> for Chapter {
    fn from(raw: ChapterRaw) -> Self {
        Self {
            id: raw.chapter_number.unwrap_or_default(),
            title: raw.title,
            novel_id: raw.novel_id,
            created_at: raw.created_at,
            updated_at: raw.created_at,
            content: raw.content,
            chapter_number: raw.chapter_number,
        }
    }
}
