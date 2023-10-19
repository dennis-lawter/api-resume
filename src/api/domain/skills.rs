use crate::api::prelude::*;

use sqlx::SqlitePool;

#[derive(sqlx::FromRow, poem_openapi::Object)]
pub struct SkillRow {
    pub id: i64,
    pub group_name: String,
    pub skill_name: String,
}

impl SkillRow {
    pub async fn get_all_by_resume_id(
        db_pool: &SqlitePool,
        resume_id: i64,
    ) -> DomainResult<Vec<SkillRow>> {
        sqlx::query_as!(
            SkillRow,
            r#"
select
	sg.id,
    sg.group_name,
    s.skill_name
from
	skill_group sg
join
	skill s
	on s.skill_group_id = sg.id
where
    sg.resume_id = $1
order by
    sg.id asc;
        "#,
            resume_id
        )
        .fetch_all(db_pool)
        .await
        .map_err(Error::SqlxError)
    }
}
