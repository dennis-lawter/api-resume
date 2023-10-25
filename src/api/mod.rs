pub mod config;
pub mod prelude;

mod action;
mod domain;
mod render;
mod static_handlers;

use action::api_v1::ApiV1;
use action::api_v1_factory::create;
use color_eyre::Result;
use config::Config;
use poem::listener::TcpListener;
use poem::middleware::AddDataEndpoint;
use poem::EndpointExt;
use poem::Route;
use poem_openapi::OpenApiService;
use sqlx::SqlitePool;

use self::prelude::*;

/// Creates and configures the resume API service with the given database pool and config.
///
/// # Parameters
/// - `db_pool`: The SQLite database connection pool.
/// - `config`: Configuration for the API.
///
/// # Returns
/// Returns a `Result` containing the constructed `ResumeApiService`.
pub fn create_resume_api(
    db_pool: SqlitePool,
    config: &Config,
) -> ApplicationResult<ResumeApiService> {
    let api_v1 = create(config)?;

    let documentation_endpoints = create_documentation_endpoints(&api_v1);

    let endpoints = Route::new()
        .nest(ApiV1::PATH_VERSION, api_v1)
        .nest("/docs", documentation_endpoints)
        .at("/", static_handlers::index)
        .at("/neofetch", static_handlers::neofetch)
        .data(db_pool);

    Ok(ResumeApiService::new(endpoints, &config.base_host))
}

/// Helper function to create documentation endpoints for the provided API version.
///
/// # Parameters
/// - `api_v1`: The version 1 of the API for which documentation endpoints should be created.
///
/// # Returns
/// Returns a `Route` containing nested documentation endpoints.
fn create_documentation_endpoints(api_v1: &OpenApiService<ApiV1, ()>) -> Route {
    Route::new()
        .nest("/swagger", api_v1.swagger_ui())
        .nest("/rapidoc", api_v1.rapidoc())
        .nest("/openapi_explorer", api_v1.openapi_explorer())
        .nest("/redoc", api_v1.redoc())
        .nest("/spec_json", api_v1.spec_endpoint())
}

/// Represents the main structure of the Resume API service.
pub struct ResumeApiService {
    endpoints: AddDataEndpoint<Route, SqlitePool>,
    base_host: String,
}

impl ResumeApiService {
    /// Creates a new instance of `ResumeApiService`.
    ///
    /// # Parameters
    /// - `endpoints`: Configured endpoints for the API.
    /// - `base_host`: Base host where the API will be served.
    fn new(endpoints: AddDataEndpoint<Route, SqlitePool>, base_host: &str) -> Self {
        Self {
            endpoints,
            base_host: base_host.to_string(),
        }
    }

    /// Starts the API service and begins listening for incoming requests.
    ///
    /// # Returns
    /// Returns a `Result` which is `Ok(())` when the server terminates successfully or contains
    /// an error if there's an issue.
    pub async fn serve(self) -> Result<()> {
        poem::Server::new(TcpListener::bind(&self.base_host))
            .run(self.endpoints)
            .await?;
        Ok(())
    }
}
