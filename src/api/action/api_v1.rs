use poem::web::Data;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use poem_openapi::Tags;
use sqlx::SqlitePool;

use crate::api::domain::contact_info::get_all_contact_infos_by_resume_id;
use crate::api::domain::contact_info::ContactInfoRow;

/// Represents the v1 version of the API.
pub struct ApiV1;

/// API tags used for categorizing endpoints.
#[allow(dead_code)]
#[derive(Tags)]
enum ApiTags {
    Info,
    Skills,
    Experience,
    Education,
}

const RESUME_ID: i64 = 1;

#[OpenApi]
impl ApiV1 {
    pub const PATH_VERSION: &'static str = "/v1";

    /// Fetches the overview of the resume.
    ///
    /// This endpoint retrieves the main overview of the resume, including general information,
    /// skills, experiences, and education.
    ///
    /// # Errors
    /// Returns a `NotFoundError` if the overview cannot be loaded.
    // // #[oai(path = "/", method = "get", tag = "ApiTags::Info")]
    // async fn overview(
    //     &self,
    //     middleware: Data<&Arc<SqlitePool>>,
    // ) -> poem::Result<Json<OverviewView>> {
    //     let pool = middleware.0.clone();
    //     let model = OverviewModel::load_by_id(pool, RESUME_ID)
    //         .await
    //         .map_err(|_| NotFoundError)?;

    //     let view = OverviewView::from(*model);

    //     Ok(Json(view))
    // }

    /// Fetches the contact info of the resume.
    #[oai(path = "/contact-info", method = "get", tag = "ApiTags::Info")]
    async fn contact_info(
        &self,
        db_pool: Data<&SqlitePool>,
    ) -> poem::Result<Json<Vec<ContactInfoRow>>> {
        let result = get_all_contact_infos_by_resume_id(db_pool.0, RESUME_ID)
            .await
            .map_err(poem::Error::from)?;
        Ok(Json(result))
    }
}
