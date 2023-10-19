use crate::api::prelude::*;

use sqlx::SqlitePool;

#[derive(sqlx::FromRow)]
pub struct ContactInfoRow {
    pub method: String,
    pub information: String,
}
impl ContactInfoRow {
    pub async fn get_all_by_resume_id(
        db_pool: &SqlitePool,
        resume_id: i64,
    ) -> DomainResult<Vec<ContactInfoRow>> {
        sqlx::query_as!(
            ContactInfoRow,
            r#"
select
    method,
    information
from
    contact_info
where
    resume_id = $1
        "#,
            resume_id
        )
        .fetch_all(db_pool)
        .await
        .map_err(Error::SqlxError)
    }
}
