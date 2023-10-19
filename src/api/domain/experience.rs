use crate::api::prelude::*;

use sqlx::SqlitePool;

use super::DomainRow;

#[derive(sqlx::FromRow, poem_openapi::Object)]
pub struct ExperienceRow {
    pub id: i64,
    pub employer: String,
    pub title: String,
    pub employment_start_date: String,
    pub employment_end_date: String,
    pub achievement_id: i64,
    pub achievement: String,
}
#[async_trait::async_trait]
impl DomainRow for ExperienceRow {
    async fn get_all_by_resume_id(
        db_pool: &SqlitePool,
        resume_id: i64,
    ) -> ApplicationResult<Vec<ExperienceRow>> {
        sqlx::query_as!(
            ExperienceRow,
            r#"
select
	pp.id,
	pp.employer,
	pp.title,
	pp.employment_start_date,
	pp.employment_end_date,
	ppa.id as 'achievement_id',
	ppa.achievement
from
	previous_position pp
join
	previous_position_achievement ppa
	on ppa.previous_position_id = pp.id
where
    pp.resume_id = $1
order by
    pp.id asc;
        "#,
            resume_id
        )
        .fetch_all(db_pool)
        .await
        .map_err(Error::SqlxError)
    }
}
