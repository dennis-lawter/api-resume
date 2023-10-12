#![feature(async_fn_in_trait)]

mod domain;

use dennis_lawter_resume_api_library::config::Config;
use dennis_lawter_resume_api_library::create_resume_api;
use dennis_lawter_resume_api_library::prelude::*;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv::dotenv()?;

    let config = Config::new()?;

    let db_pool = get_db_pool(config.database_url.as_str()).await?;
    migrate_db(&db_pool).await?;

    let resume_api = create_resume_api(db_pool, &config)?;
    resume_api.serve().await?;

    Ok(())
}

async fn get_db_pool(db_url: &str) -> Result<SqlitePool, Error> {
    SqlitePool::connect(db_url).await.map_err(Error::from)
}

async fn migrate_db(pool: &SqlitePool) -> Result<(), Error> {
    sqlx::migrate!().run(pool).await.map_err(Error::from)
}
