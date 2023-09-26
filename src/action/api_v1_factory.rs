use poem_openapi::OpenApiService;

use color_eyre::Result;

use super::api_v1::ApiV1;

pub fn api_v1_factory() -> Result<OpenApiService<ApiV1, ()>> {
    let description = std::env!("CARGO_PKG_DESCRIPTION");
    let version = std::env!("CARGO_PKG_VERSION");
    let server_path = get_server_path()?;
    Ok(OpenApiService::new(ApiV1, description, version).server(server_path))
}

fn get_server_path() -> Result<String> {
    let base_url = std::env::var("BASE_URL")?;
    Ok(format!("{}{}", base_url, ApiV1::PATH_VERSION))
}
