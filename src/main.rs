mod api;

use api::config::Config;
use api::create_resume_api;
use api::prelude::*;

use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> EyreResult<()> {
    color_eyre::install()?;
    dotenv::dotenv()?;

    let config = Config::new()?;

    let db_pool = get_db_pool(config.database_url.as_str()).await?;
    migrate_db(&db_pool).await?;

    let resume_api = create_resume_api(db_pool, &config)?;
    resume_api.serve().await?;

    Ok(())
}

async fn get_db_pool(db_url: &str) -> EyreResult<SqlitePool, Error> {
    SqlitePool::connect(db_url).await.map_err(Error::from)
}

async fn migrate_db(pool: &SqlitePool) -> EyreResult<(), Error> {
    sqlx::migrate!().run(pool).await.map_err(Error::from)
}
