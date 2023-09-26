use poem::error::NotFoundError;
use poem::web::Data;
use poem_openapi::param::Query;
use poem_openapi::payload::Json;
use poem_openapi::payload::PlainText;
use poem_openapi::OpenApi;
use poem_openapi::Tags;
use sqlx::SqlitePool;

use crate::domain::contact_info::ContactInfoDao;
use crate::render::contact_info::ContactInfoView;

pub struct ApiV1;

#[derive(Tags)]
enum ApiTags {
    Info,
    Skills,
    Experience,
    Education,
}

#[OpenApi]
impl ApiV1 {
    pub const PATH_VERSION: &'static str = "/v1";

    /// Hello World
    #[oai(path = "/hello", method = "get")]
    async fn hello(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello world!".to_string()),
        }
    }

    /// Contact Information
    #[oai(path = "/contact", method = "get", tag = "ApiTags::Info")]
    async fn contact_all(
        &self,
        pool: Data<&SqlitePool>,
    ) -> poem::Result<Json<Vec<ContactInfoView>>> {
        let contact_infos = ContactInfoDao::retrieve_all(pool.0)
            .await
            .map_err(|_| NotFoundError)?;

        let contact_info_views: Vec<ContactInfoView> = contact_infos
            .into_iter()
            .map(|dao| ContactInfoView::from_domain(dao))
            .collect();

        Ok(Json(contact_info_views))
    }
}
