use std::sync::Arc;

use async_trait::async_trait;
use color_eyre::eyre::Result;
use sqlx::SqlitePool;

use super::Dao;
use super::DomainError;
use super::StaticModel;

/// Represents the contact information model in the domain.
#[derive(Clone)]
pub struct ContactInfoModel {
    pub id: i64,
    pub method: String,
    pub information: String,
}

impl From<ContactInfoDao> for ContactInfoModel {
    fn from(dao: ContactInfoDao) -> Self {
        Self {
            id: dao.id,
            method: dao.method,
            information: dao.information,
        }
    }
}

#[async_trait]
impl StaticModel for ContactInfoModel {
    async fn load_all(pool: Arc<SqlitePool>) -> Result<Vec<Box<Self>>> {
        let daos = ContactInfoDao::retrieve_all(pool).await?;
        Ok(daos
            .into_iter()
            .map(|dao| Box::new(Self::from(*dao)))
            .collect())
    }

    async fn load_by_id(pool: Arc<SqlitePool>, id: i64) -> Result<Box<Self>> {
        let dao = ContactInfoDao::retrieve_by_id(pool, id).await?;
        Ok(Box::new(Self::from(*dao)))
    }
}

/// Represents the data access object for contact information.
#[derive(sqlx::FromRow, Clone)]
pub struct ContactInfoDao {
    pub id: i64,
    pub method: String,
    pub information: String,
}

#[async_trait]
impl Dao for ContactInfoDao {
    async fn retrieve_all(pool: Arc<SqlitePool>) -> Result<Vec<Box<ContactInfoDao>>> {
        let contact_infos = sqlx::query_as!(
            ContactInfoDao,
            r#"
select
    id,
    method,
    information
from
    contact_info
order by
    id asc
            "#,
        )
        .fetch_all(pool.as_ref())
        .await
        .map_err(DomainError::SqlxError)?;

        Ok(contact_infos.into_iter().map(Box::new).collect())
    }

    async fn retrieve_by_id(pool: Arc<SqlitePool>, id: i64) -> Result<Box<ContactInfoDao>> {
        let rows = sqlx::query_as!(
            ContactInfoDao,
            r#"
select
    id,
    method,
    information
from
    contact_info
where
    id = $1
            "#,
            id
        )
        .fetch_all(pool.as_ref())
        .await
        .map_err(DomainError::SqlxError)?;

        rows.get(0)
            .cloned()
            .map(Box::new)
            .ok_or(DomainError::NoResult.into())
    }
}
