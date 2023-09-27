pub mod contact_info;
pub mod experience;
pub mod overview;

use async_trait::async_trait;
use color_eyre::eyre::Result;
use sqlx::SqlitePool;
use thiserror::Error;

#[derive(Debug, Error)]
enum DomainError {
    #[error("No result found")]
    NoResult,
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

#[async_trait]
pub trait StaticModel {
    async fn load_all(pool: &SqlitePool) -> Result<Vec<Box<Self>>>;
    async fn load_by_id(pool: &SqlitePool, id: i64) -> Result<Box<Self>>;
}

#[async_trait]
trait Dao {
    async fn retrieve_all(pool: &SqlitePool) -> Result<Vec<Box<Self>>>;
    async fn retrieve_by_id(pool: &SqlitePool, id: i64) -> Result<Box<Self>>;
}
