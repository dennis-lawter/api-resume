use crate::api::config::Config;
use crate::api::prelude::ApplicationResult;

use super::api_v1::ApiV1;

use poem_openapi::OpenApiService;

/// Constructs and configures the version 1 API.
///
/// # Parameters
/// * `config` - A reference to the application's configuration.
///
/// # Returns
/// Returns a `Result` containing the configured `OpenApiService` for version 1 of the API.
pub fn create(config: &Config) -> ApplicationResult<OpenApiService<ApiV1, ()>> {
    let server_path = format!("{}{}", config.base_url, ApiV1::PATH_VERSION);
    Ok(OpenApiService::new(
        ApiV1,
        &config.cargo_pkg_description,
        &config.cargo_pkg_version,
    )
    .server(server_path))
}
