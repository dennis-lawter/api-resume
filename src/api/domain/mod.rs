use sqlx::SqlitePool;

use super::prelude::*;

pub mod contact_info;
pub mod education;
pub mod experience;
pub mod overview;
pub mod skills;

#[async_trait::async_trait]
pub trait DomainRow {
    async fn get_all_by_resume_id(
        db_pool: &SqlitePool,
        resume_id: i64,
    ) -> ApplicationResult<Vec<Self>>
    where
        Self: Sized + Send + 'static;
}
