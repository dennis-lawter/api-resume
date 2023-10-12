use std::sync::Arc;

use poem::error::NotFoundError;
use poem::web::Data;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use poem_openapi::Tags;
use sqlx::SqlitePool;

use crate::domain::overview::OverviewModel;
use crate::domain::StaticModel;
use crate::render::overview::OverviewView;

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
    #[oai(path = "/", method = "get", tag = "ApiTags::Info")]
    async fn overview(
        &self,
        middleware: Data<&Arc<SqlitePool>>,
    ) -> poem::Result<Json<OverviewView>> {
        let pool = middleware.0.clone();
        let model = OverviewModel::load_by_id(pool, RESUME_ID)
            .await
            .map_err(|_| NotFoundError)?;

        let view = OverviewView::from(*model);

        Ok(Json(view))
    }
}
