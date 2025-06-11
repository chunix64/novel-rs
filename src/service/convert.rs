use crate::{
    db::models::{Chapter, Post},
    site::content::novels::{ChapterRaw, NovelRaw},
};

pub fn novel_raw_to_post(raw: NovelRaw, content_type_id: i64) -> Post {
    Post {
        id: raw.id,
        content_type_id,
        title: raw.title,
        slug: raw.slug,
        thumbnail: raw.thumbnail,
        description: raw.description,
        created_at: raw.created_at,
        updated_at: raw.updated_at,
    }
}

impl From<ChapterRaw> for Chapter {
    fn from(raw: ChapterRaw) -> Self {
        Self {
            id: raw.chapter_number.unwrap_or_default(),
            title: raw.title,
            slug: raw.slug,
            post_id: raw.novel_id,
            created_at: raw.created_at,
            updated_at: raw.created_at,
            content: raw.content,
            chapter_number: raw.chapter_number,
        }
    }
}
