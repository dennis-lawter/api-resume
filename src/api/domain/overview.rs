use crate::api::prelude::*;

use sqlx::SqlitePool;

#[derive(sqlx::FromRow)]
pub struct OverviewRow {
    pub full_name: String,
    pub title: String,
    pub objective: String,
}
impl OverviewRow {
    pub async fn get_all_by_resume_id(
        db_pool: &SqlitePool,
        resume_id: i64,
    ) -> DomainResult<OverviewRow> {
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
        .fetch_one(db_pool)
        .await
        .map_err(Error::SqlxError)
    }
}
