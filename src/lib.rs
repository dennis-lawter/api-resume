mod action;
mod domain;
mod render;

use std::sync::Arc;

use action::api_v1::ApiV1;
use action::api_v1_factory::api_v1_factory;
use color_eyre::Result;
use poem::handler;
use poem::http::StatusCode;
use poem::listener::TcpListener;
use poem::middleware::AddDataEndpoint;
use poem::web::Html;
use poem::EndpointExt;
use poem::Route;
use sqlx::SqlitePool;

#[handler]
async fn index() -> poem::Result<Html<String>, poem::Error> {
    let content = tokio::fs::read_to_string("static/index.html")
        .await
        .map_err(|err| poem::Error::new(err, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Html(content))
}

pub fn resume_api_factory(pool: SqlitePool) -> Result<ResumeApi> {
    let api_v1 = api_v1_factory()?;
    let swagger = api_v1.swagger_ui();
    let rapidoc = api_v1.rapidoc();
    let openapi_explorer = api_v1.openapi_explorer();
    let redoc = api_v1.redoc();
    // let spec_yaml = api_v1.spec_endpoint_yaml();
    let spec_json = api_v1.spec_endpoint();
    let endpoints = Route::new()
        .nest(ApiV1::PATH_VERSION, api_v1)
        .nest("/swagger", swagger)
        .nest("/rapidoc", rapidoc)
        .nest("/openapi_explorer", openapi_explorer)
        .nest("/redoc", redoc)
        // .nest("/spec_yaml", spec_yaml) // not great because it downloads rather than renders in the page...
        .nest("/spec_json", spec_json)
        .at("/", index)
        .data(Arc::new(pool));
    Ok(ResumeApi::new(endpoints))
}

pub struct ResumeApi {
    endpoints: AddDataEndpoint<Route, Arc<SqlitePool>>,
}
impl ResumeApi {
    fn new(endpoints: AddDataEndpoint<Route, Arc<SqlitePool>>) -> Self {
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
