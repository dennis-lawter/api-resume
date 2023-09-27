use async_trait::async_trait;
use color_eyre::eyre::Result;
use sqlx::SqlitePool;

use super::Dao;
use super::DomainError;
use super::StaticModel;

pub struct OverviewModel {
    pub id: i64,
    pub full_name: String,
    pub title: String,
    pub objective: String,
}
impl From<OverviewDao> for OverviewModel {
    fn from(dao: OverviewDao) -> Self {
        Self {
            id: dao.id,
            full_name: dao.full_name,
            title: dao.title,
            objective: dao.objective,
        }
    }
}
#[async_trait]
impl StaticModel for OverviewModel {
    async fn load_all(pool: &SqlitePool) -> Result<Vec<Box<Self>>> {
        let daos = OverviewDao::retrieve_all(pool).await?;
        Ok(daos
            .into_iter()
            .map(|dao| Box::new(Self::from(*dao)))
            .collect())
    }

    async fn load_by_id(pool: &SqlitePool, id: i64) -> Result<Box<Self>> {
        let dao = OverviewDao::retrieve_by_id(pool, id).await?;
        Ok(Box::new(Self::from(*dao)))
    }
}

#[derive(sqlx::FromRow, Clone)]
struct OverviewDao {
    pub id: i64,
    pub full_name: String,
    pub title: String,
    pub objective: String,
}

#[async_trait]
impl Dao for OverviewDao {
    async fn retrieve_all(pool: &SqlitePool) -> Result<Vec<Box<OverviewDao>>> {
        let query_result = sqlx::query_as!(
            OverviewDao,
            "select
                id,
                full_name,
                title,
                objective
            from
                resume_overview
            order by
                id asc"
        )
        .fetch_all(pool)
        .await;

        match query_result {
            Ok(rows) => {
                let boxed_rows: Vec<Box<OverviewDao>> = rows.into_iter().map(Box::new).collect();

                Ok(boxed_rows)
            }
            Err(err) => Err(err.into()),
        }
    }

    async fn retrieve_by_id(pool: &SqlitePool, id: i64) -> Result<Box<OverviewDao>> {
        let query_result = sqlx::query_as!(
            OverviewDao,
            "SELECT
            id,
            full_name,
            title,
            objective
        FROM
            resume_overview
        WHERE
            id = $1",
            id
        )
        .fetch_all(pool)
        .await;

        match query_result {
            Ok(rows) => rows
                .get(0)
                .cloned()
                .map(Box::new)
                .ok_or(DomainError::NoResult.into()),
            Err(err) => Err(DomainError::SqlxError(err).into()),
        }
    }
}
