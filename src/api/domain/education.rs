use crate::api::prelude::*;

use sqlx::SqlitePool;

use super::DomainRow;

#[derive(sqlx::FromRow)]
pub struct EducationRow {
    pub id: i64,
    pub school: String,
    pub degree: String,
    pub education_start_date: String,
    pub education_end_date: String,
}
#[async_trait::async_trait]
impl DomainRow for EducationRow {
    async fn get_all_by_resume_id(
        db_pool: &SqlitePool,
        resume_id: i64,
    ) -> ApplicationResult<Vec<EducationRow>> {
        sqlx::query_as!(
            EducationRow,
            r#"
select
    id,
    school,
    degree,
    education_start_date,
    education_end_date
from
    education
where
    resume_id = $1
order by
    id asc
        "#,
            resume_id
        )
        .fetch_all(db_pool)
        .await
        .map_err(Error::SqlxError)
    }
}
