use color_eyre::eyre::Result;
use sqlx::SqlitePool;

#[derive(sqlx::FromRow)]
pub struct ContactInfoDao {
    pub id: i64,
    pub method: String,
    pub information: String,
}

impl ContactInfoDao {
    #[allow(dead_code)] // rustc isn't following the oai binding to here
    pub async fn retrieve_all(pool: &SqlitePool) -> Result<Vec<ContactInfoDao>> {
        let contact_infos_result: Result<Vec<ContactInfoDao>, sqlx::Error> = sqlx::query_as!(
            ContactInfoDao,
            "SELECT id, method, information FROM contact_info"
        )
        .fetch_all(pool)
        .await;

        Ok(contact_infos_result?)
    }
}
