pub mod contact_info;
pub mod overview;

use async_trait::async_trait;
use color_eyre::eyre::Result;
use sqlx::SqlitePool;

#[async_trait]
pub trait Dao {
    async fn retrieve_all(pool: &SqlitePool) -> Result<Vec<Box<Self>>>;
}
