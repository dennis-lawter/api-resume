pub mod contact_info;
pub mod experience;
pub mod overview;

use async_trait::async_trait;
use color_eyre::eyre::Result;
use sqlx::SqlitePool;
use thiserror::Error;

use std::sync::Arc;

/// Custom error types for domain operations.
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("No result found")]
    NoResult,
    #[error("Database connection not provided")]
    DbMissing,
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

/// Represents models with static data that can be fetched without any dynamic input.
#[async_trait]
pub trait StaticModel {
    async fn load_all(pool: Arc<SqlitePool>) -> Result<Vec<Box<Self>>>;
    async fn load_by_id(pool: Arc<SqlitePool>, id: i64) -> Result<Box<Self>>;
}

/// Data access operations for domain entities.
///
/// This trait defines the fundamental database operations
/// that domain entities must support.
#[async_trait]
trait Dao {
    async fn retrieve_all(pool: Arc<SqlitePool>) -> Result<Vec<Box<Self>>>;
    async fn retrieve_by_id(pool: Arc<SqlitePool>, id: i64) -> Result<Box<Self>>;
}
