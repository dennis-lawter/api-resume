use super::api_v1::ApiV1;
use crate::config::Config;
use color_eyre::Result;
use poem_openapi::OpenApiService;

/// Constructs and configures the version 1 API.
///
/// # Arguments
/// * `config` - A reference to the application's configuration.
///
/// # Returns
/// Returns a `Result` containing the configured `OpenApiService` for version 1 of the API.
pub fn create(config: &Config) -> Result<OpenApiService<ApiV1, ()>> {
    let server_path = format!("{}{}", config.base_url, ApiV1::PATH_VERSION);
    Ok(OpenApiService::new(
        ApiV1,
        &config.cargo_pkg_description,
        &config.cargo_pkg_version,
    )
    .server(server_path))
}
