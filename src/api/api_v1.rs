use poem_openapi::param::Query;
use poem_openapi::payload::PlainText;
use poem_openapi::OpenApi;
use poem_openapi::OpenApiService;

use color_eyre::Result;

pub struct ApiV1;

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

#[OpenApi]
impl ApiV1 {
    pub const PATH_VERSION: &'static str = "/v1";
    /// Hello World
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello world!".to_string()),
        }
    }
}
