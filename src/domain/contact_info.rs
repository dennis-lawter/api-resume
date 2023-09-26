use async_trait::async_trait;
use color_eyre::eyre::Result;
use sqlx::SqlitePool;

use super::Dao;

#[derive(sqlx::FromRow)]
pub struct ContactInfoDao {
    pub id: i64,
    pub method: String,
    pub information: String,
}

#[async_trait]
impl Dao for ContactInfoDao {
    async fn retrieve_all(pool: &SqlitePool) -> Result<Vec<Box<ContactInfoDao>>> {
        let contact_infos_result: Result<Vec<ContactInfoDao>, sqlx::Error> = sqlx::query_as!(
            ContactInfoDao,
            "select
                id,
                method,
                information
            from
                contact_info
            order by
                id asc"
        )
        .fetch_all(pool)
        .await;

        match contact_infos_result {
            Ok(contact_infos) => {
                let boxed_contact_infos: Vec<Box<ContactInfoDao>> =
                    contact_infos.into_iter().map(Box::new).collect();

                Ok(boxed_contact_infos)
            }
            Err(err) => Err(err.into()),
        }
    }
}
