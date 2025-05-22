pub mod convert;
pub mod novel;

pub trait SiteService: Send + Sync {
    async fn run(&self) {}
}
