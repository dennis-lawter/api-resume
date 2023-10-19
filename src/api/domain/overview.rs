use crate::api::prelude::*;

use sqlx::SqlitePool;

use super::DomainRow;

#[derive(sqlx::FromRow, Clone)]
pub struct OverviewRow {
    pub full_name: String,
    pub title: String,
    pub objective: String,
}
#[async_trait::async_trait]
impl DomainRow for OverviewRow {
    async fn get_all_by_resume_id(
        db_pool: &SqlitePool,
        resume_id: i64,
    ) -> ApplicationResult<Vec<OverviewRow>> {
        sqlx::query_as!(
            OverviewRow,
            r#"
select
    full_name,
    title,
    objective
from
    resume_overview
where
    id = $1
        "#,
            resume_id
        )
        .fetch_all(db_pool)
        .await
        .map_err(Error::SqlxError)
    }
}
