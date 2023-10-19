use crate::api::prelude::*;

use sqlx::SqlitePool;

#[derive(sqlx::FromRow, poem_openapi::Object)]
pub struct ContactInfoRow {
    pub id: i64,
    pub method: String,
    pub information: String,
    pub resume_id: i64,
}
pub async fn get_all_contact_infos_by_resume_id(
    db_pool: &SqlitePool,
    resume_id: i64,
) -> DomainResult<Vec<ContactInfoRow>> {
    sqlx::query_as!(
        ContactInfoRow,
        r#"
select
    *
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
