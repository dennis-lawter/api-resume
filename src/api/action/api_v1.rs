use poem::http::StatusCode;
use poem::web::Data;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use poem_openapi::Tags;
use sqlx::SqlitePool;

use crate::api::domain::DomainRow;
use crate::api::prelude::ApplicationResult;
use crate::api::render::contact_info::ContactInfoView;
use crate::api::render::education::EducationView;
use crate::api::render::experience::ExperienceView;
use crate::api::render::overview::OverviewView;
use crate::api::render::skills::SkillView;
use crate::api::render::View;

/// Represents the v1 version of the API.
pub struct ApiV1;

/// API tags used for categorizing endpoints.
#[allow(dead_code)]
#[derive(Tags)]
enum ApiTags {
    Abstract,
    Details,
}

const RESUME_ID: i64 = 1;

async fn fetch_and_render<'a, R, V>(db_pool: &SqlitePool) -> ApplicationResult<Vec<V>>
where
    R: DomainRow + std::marker::Send + 'static,
    V: View<R> + std::marker::Send + 'static,
{
    let rows: _ = R::get_all_by_resume_id(db_pool, RESUME_ID).await?;
    Ok(V::from_collection(rows))
}

#[OpenApi]
impl ApiV1 {
    pub const PATH_VERSION: &'static str = "/v1";

    /// Fetches the overview of the resume.
    ///
    /// This endpoint retrieves the main overview of the resume, including general information,
    /// skills, experiences, and education.
    #[oai(path = "/", method = "get", tag = "ApiTags::Abstract")]
    async fn overview(&self, db_pool: Data<&SqlitePool>) -> poem::Result<Json<OverviewView>> {
        let view: Vec<OverviewView> = fetch_and_render(db_pool.0).await?;
        if view.len() <= 0 {
            return Err(poem::Error::from_status(StatusCode::NOT_FOUND));
        }

        let mut first_view = view[0].clone();
        first_view.contact_info = fetch_and_render(db_pool.0).await?;
        first_view.skills = fetch_and_render(db_pool.0).await?;
        first_view.experience = fetch_and_render(db_pool.0).await?;
        first_view.education = fetch_and_render(db_pool.0).await?;

        Ok(Json(first_view))
    }

    /// Fetches the contact info of the resume.
    #[oai(path = "/contact-info", method = "get", tag = "ApiTags::Details")]
    async fn contact_info(
        &self,
        db_pool: Data<&SqlitePool>,
    ) -> poem::Result<Json<Vec<ContactInfoView>>> {
        let view = fetch_and_render(db_pool.0).await?;
        Ok(Json(view))
    }

    /// Fetches experience with listed achievements.
    #[oai(path = "/experience", method = "get", tag = "ApiTags::Details")]
    async fn experience(
        &self,
        db_pool: Data<&SqlitePool>,
    ) -> poem::Result<Json<Vec<ExperienceView>>> {
        let view = fetch_and_render(db_pool.0).await?;
        Ok(Json(view))
    }

    /// Fetches education history.
    #[oai(path = "/education", method = "get", tag = "ApiTags::Details")]
    async fn education(
        &self,
        db_pool: Data<&SqlitePool>,
    ) -> poem::Result<Json<Vec<EducationView>>> {
        let view = fetch_and_render(db_pool.0).await?;
        Ok(Json(view))
    }

    /// Fetches skills, grouped by their shared skill groups.
    #[oai(path = "/skills", method = "get", tag = "ApiTags::Details")]
    async fn skills(&self, db_pool: Data<&SqlitePool>) -> poem::Result<Json<Vec<SkillView>>> {
        let view = fetch_and_render(db_pool.0).await?;
        Ok(Json(view))
    }
}
