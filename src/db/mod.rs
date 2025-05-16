use repository::{artist::ArtistRepository, author::AuthorRepository};
use sqlx::SqlitePool;

mod repository;

pub mod models;
pub mod schema;

pub struct DB {
    pool: SqlitePool,
    pub author: AuthorRepository,
    pub artist: ArtistRepository,
}

impl DB {
    pub fn new(pool: SqlitePool) -> Self {
        let author = AuthorRepository::new(pool.clone());
        let artist = ArtistRepository::new(pool.clone());
        Self {
            pool,
            author,
            artist,
        }
    }
}
