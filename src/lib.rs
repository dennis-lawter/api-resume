mod action;
mod domain;
mod render;

use action::api_v1::ApiV1;
use action::api_v1_factory::api_v1_factory;
use color_eyre::Result;
use poem::listener::TcpListener;
use poem::middleware::AddDataEndpoint;
use poem::EndpointExt;
use poem::Route;
use sqlx::SqlitePool;

pub fn resume_api_factory(pool: SqlitePool) -> Result<ResumeApi> {
    let api_v1 = api_v1_factory()?;
    let docs = api_v1.swagger_ui();
    let endpoints = Route::new()
        .nest(ApiV1::PATH_VERSION, api_v1)
        .nest("/docs", docs)
        .data(pool);
    Ok(ResumeApi::new(endpoints))
}

pub struct ResumeApi {
    endpoints: AddDataEndpoint<Route, SqlitePool>,
}
impl ResumeApi {
    fn new(endpoints: AddDataEndpoint<Route, SqlitePool>) -> Self {
        Self { endpoints }
    }

    pub async fn serve(self) -> Result<()> {
        let base_url = std::env::var("BASE_HOST")?;
        poem::Server::new(TcpListener::bind(base_url))
            .run(self.endpoints)
            .await?;
        Ok(())
    }
}
