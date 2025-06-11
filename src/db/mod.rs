use repository::{
    artist::ArtistRepository, author::AuthorRepository, chapter::ChapterRepository,
    post::PostRepository, post_tag::PostTagRepository, tag::TagRepository,
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
    pub post: PostRepository,
    pub post_tag: PostTagRepository,
    pub chapter: ChapterRepository,
}

impl Database {
    pub fn new(pool: SqlitePool) -> Self {
        let author = AuthorRepository::new(pool.clone());
        let artist = ArtistRepository::new(pool.clone());
        let tag = TagRepository::new(pool.clone());
        let post = PostRepository::new(pool.clone());
        let post_tag = PostTagRepository::new(pool.clone());
        let chapter = ChapterRepository::new(pool.clone());
        Self {
            pool,
            author,
            artist,
            tag,
            post,
            post_tag,
            chapter,
        }
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
