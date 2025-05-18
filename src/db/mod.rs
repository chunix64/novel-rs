use repository::{
    artist::ArtistRepository, author::AuthorRepository, chapter::ChapterRepository,
    novel::NovelRepository, novel_tag::NovelTagRepository, tag::TagRepository,
};
use sqlx::SqlitePool;

mod repository;

pub mod models;
pub mod schema;

pub struct Database {
    pool: SqlitePool,
    pub author: AuthorRepository,
    pub artist: ArtistRepository,
    pub tag: TagRepository,
    pub novel: NovelRepository,
    pub novel_tag: NovelTagRepository,
    pub chapter: ChapterRepository,
}

impl Database {
    pub fn new(pool: SqlitePool) -> Self {
        let author = AuthorRepository::new(pool.clone());
        let artist = ArtistRepository::new(pool.clone());
        let tag = TagRepository::new(pool.clone());
        let novel = NovelRepository::new(pool.clone());
        let novel_tag = NovelTagRepository::new(pool.clone());
        let chapter = ChapterRepository::new(pool.clone());
        Self {
            pool,
            author,
            artist,
            tag,
            novel,
            novel_tag,
            chapter,
        }
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
