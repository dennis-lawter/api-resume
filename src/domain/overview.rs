use std::sync::Arc;

use async_trait::async_trait;
use color_eyre::eyre::Result;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use sqlx::SqlitePool;

use super::contact_info::ContactInfoModel;
use super::Dao;
use super::DomainError;
use super::StaticModel;

pub struct OverviewModel {
    pub pool: Option<Arc<SqlitePool>>,

    pub id: i64,
    pub full_name: String,
    pub title: String,
    pub objective: String,

    pub contact_infos: Vec<Box<ContactInfoModel>>,
}

impl OverviewModel {
    pub fn with_pool(mut self, pool: Arc<SqlitePool>) -> Self {
        self.pool = Some(pool);
        self
    }

    pub fn from_dao(mut self, dao: OverviewDao) -> Self {
        self.id = dao.id;
        self.full_name = dao.full_name;
        self.title = dao.title;
        self.objective = dao.objective;
        self
    }

    pub async fn with_related(mut self) -> Result<Self> {
        let pool = self.pool.as_ref().ok_or(DomainError::DbMissing)?;
        let contact_infos = ContactInfoModel::load_all(pool.clone()).await?;
        self.contact_infos = contact_infos;

        Ok(self)
    }
}

impl Default for OverviewModel {
    fn default() -> Self {
        Self {
            pool: None,
            id: 0,
            full_name: String::new(),
            title: String::new(),
            objective: String::new(),
            contact_infos: Vec::new(),
        }
    }
}

#[async_trait]
impl StaticModel for OverviewModel {
    async fn load_all(pool: Arc<SqlitePool>) -> Result<Vec<Box<Self>>> {
        let daos = OverviewDao::retrieve_all(pool.clone()).await?;

        let mut futures = FuturesUnordered::new();
        for dao in daos {
            let model_pool_clone = pool.clone();
            let future = tokio::task::spawn(async move {
                OverviewModel::default()
                    .with_pool(model_pool_clone)
                    .from_dao(*dao)
                    .with_related()
                    .await
            });
            futures.push(future);
        }

        let mut models = Vec::new();
        while let Some(join_result) = futures.next().await {
            let inner_result = join_result?;
            models.push(Box::new(inner_result?));
        }

        Ok(models)
    }

    async fn load_by_id(pool: Arc<SqlitePool>, id: i64) -> Result<Box<Self>> {
        let dao = OverviewDao::retrieve_by_id(pool.clone(), id).await?;

        let model_future = tokio::task::spawn(async move {
            OverviewModel::default()
                .with_pool(pool.clone())
                .from_dao(*dao)
                .with_related()
                .await
        });

        let inner_result = model_future.await??; // Handle both JoinError and your custom error
        Ok(Box::new(inner_result))
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct OverviewDao {
    pub id: i64,
    pub full_name: String,
    pub title: String,
    pub objective: String,
}

#[async_trait]
impl Dao for OverviewDao {
    async fn retrieve_all(pool: Arc<SqlitePool>) -> Result<Vec<Box<OverviewDao>>> {
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
        .fetch_all(pool.as_ref())
        .await;

        match query_result {
            Ok(rows) => {
                let boxed_rows: Vec<Box<OverviewDao>> = rows.into_iter().map(Box::new).collect();

                Ok(boxed_rows)
            }
            Err(err) => Err(err.into()),
        }
    }

    async fn retrieve_by_id(pool: Arc<SqlitePool>, id: i64) -> Result<Box<OverviewDao>> {
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
        .fetch_all(pool.as_ref())
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
