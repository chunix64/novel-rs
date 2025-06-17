use repository::{
    artist::ArtistRepository, author::AuthorRepository, chapter::ChapterRepository,
    post::PostRepository, post_author::PostAuthorRepository, post_tag::PostTagRepository,
    tag::TagRepository,
};
use sqlx::SqlitePool;

use crate::db::repository::post_artist::PostArtistRepository;

mod repository;

pub mod models;
pub mod schema;

pub struct Database {
    pub post: PostRepository,
    pub chapter: ChapterRepository,
    pub author: AuthorRepository,
    pub artist: ArtistRepository,
    pub tag: TagRepository,
    pub post_tag: PostTagRepository,
    pub post_author: PostAuthorRepository,
    pub post_artist: PostArtistRepository,
}

impl Database {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            post: PostRepository::new(pool.clone()),
            chapter: ChapterRepository::new(pool.clone()),
            author: AuthorRepository::new(pool.clone()),
            artist: ArtistRepository::new(pool.clone()),
            tag: TagRepository::new(pool.clone()),
            post_tag: PostTagRepository::new(pool.clone()),
            post_author: PostAuthorRepository::new(pool.clone()),
            post_artist: PostArtistRepository::new(pool.clone()),
        }
    }
}
