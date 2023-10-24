mod api;

use std::str::FromStr;

use api::config::Config;
use api::create_resume_api;
use api::prelude::*;

use sqlx::sqlite::SqliteConnectOptions;
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
    let options = SqliteConnectOptions::from_str(format!("sqlite://{}", db_url).as_str())?
        .create_if_missing(true);

    SqlitePool::connect_with(options).await.map_err(Error::from)
}

async fn migrate_db(pool: &SqlitePool) -> EyreResult<(), Error> {
    sqlx::migrate!().run(pool).await.map_err(Error::from)
}
