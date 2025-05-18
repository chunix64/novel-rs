use crate::{db::models::Novel, site::content::novels::NovelRaw};

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
