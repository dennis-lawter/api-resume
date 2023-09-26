use color_eyre::Result;
use dennis_lawter_resume_api_library::resume_api_factory;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<()> {
    init()?;

    let pool = get_db_pool().await?;
    migrate_db(&pool).await?;

    let resume_api = resume_api_factory(pool)?;
    resume_api.serve().await?;

    Ok(())
}

fn init() -> Result<()> {
    color_eyre::install()?;
    dotenv::dotenv()?;

    Ok(())
}

async fn get_db_pool() -> Result<SqlitePool> {
    let db_file = std::env::var("DATABASE_URL")?;
    Ok(SqlitePool::connect(db_file.as_str()).await?)
}
async fn migrate_db(pool: &SqlitePool) -> Result<()> {
    Ok(sqlx::migrate!().run(pool).await?)
}
