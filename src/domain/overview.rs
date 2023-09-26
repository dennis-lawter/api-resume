use async_trait::async_trait;
use color_eyre::eyre::Result;
use sqlx::SqlitePool;

use super::Dao;

#[derive(sqlx::FromRow)]
pub struct OverviewDao {
    pub id: i64,
    pub full_name: String,
    pub title: String,
    pub objective: String,
}

#[async_trait]
impl Dao for OverviewDao {
    async fn retrieve_all(pool: &SqlitePool) -> Result<Vec<Box<OverviewDao>>> {
        let contact_infos_result: Result<Vec<OverviewDao>, sqlx::Error> = sqlx::query_as!(
            OverviewDao,
            "select id, full_name, title, objective from resume_overview"
        )
        .fetch_all(pool)
        .await;

        match contact_infos_result {
            Ok(contact_infos) => {
                let boxed_contact_infos: Vec<Box<OverviewDao>> =
                    contact_infos.into_iter().map(Box::new).collect();

                Ok(boxed_contact_infos)
            }
            Err(err) => Err(err.into()),
        }
    }
}
