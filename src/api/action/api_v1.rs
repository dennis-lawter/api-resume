use poem::web::Data;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use poem_openapi::Tags;
use sqlx::SqlitePool;

use crate::api::domain::contact_info::ContactInfoRow;
use crate::api::domain::education::EducationRow;
use crate::api::domain::experience::ExperienceRow;
use crate::api::domain::overview::OverviewRow;
use crate::api::domain::skills::SkillRow;
use crate::api::render::contact_info::ContactInfoView;
use crate::api::render::education::EducationView;
use crate::api::render::experience::ExperienceView;
use crate::api::render::overview::OverviewView;
use crate::api::render::skills::SkillView;

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
    #[oai(path = "/", method = "get", tag = "ApiTags::Info")]
    async fn overview(&self, db_pool: Data<&SqlitePool>) -> poem::Result<Json<OverviewView>> {
        let overview_row = OverviewRow::get_all_by_resume_id(db_pool.0, RESUME_ID)
            .await
            .map_err(poem::Error::from)?;
        let ci = ContactInfoRow::get_all_by_resume_id(db_pool.0, RESUME_ID)
            .await
            .map_err(poem::Error::from)?;
        let ci_view = ContactInfoView::from_collection(ci);
        let exp = ExperienceRow::get_all_by_resume_id(db_pool.0, RESUME_ID)
            .await
            .map_err(poem::Error::from)?;
        let exp_view = ExperienceView::from_row_collection(exp);
        let edu = EducationRow::get_all_by_resume_id(db_pool.0, RESUME_ID)
            .await
            .map_err(poem::Error::from)?;
        let edu_view = EducationView::from_collection(edu);
        let skills = SkillRow::get_all_by_resume_id(db_pool.0, RESUME_ID)
            .await
            .map_err(poem::Error::from)?;
        let skill_view = SkillView::from_row_collection(skills);

        let full_view = OverviewView::new(overview_row, ci_view, skill_view, exp_view, edu_view);

        Ok(Json(full_view))
    }

    /// Fetches the contact info of the resume.
    #[oai(path = "/contact-info", method = "get", tag = "ApiTags::Info")]
    async fn contact_info(
        &self,
        db_pool: Data<&SqlitePool>,
    ) -> poem::Result<Json<Vec<ContactInfoView>>> {
        let ci = ContactInfoRow::get_all_by_resume_id(db_pool.0, RESUME_ID)
            .await
            .map_err(poem::Error::from)?;
        let view = ContactInfoView::from_collection(ci);
        Ok(Json(view))
    }

    /// Fetches experience with listed achievements.
    #[oai(path = "/experience", method = "get", tag = "ApiTags::Experience")]
    async fn experience(
        &self,
        db_pool: Data<&SqlitePool>,
    ) -> poem::Result<Json<Vec<ExperienceView>>> {
        let exp = ExperienceRow::get_all_by_resume_id(db_pool.0, RESUME_ID)
            .await
            .map_err(poem::Error::from)?;
        let view = ExperienceView::from_row_collection(exp);
        Ok(Json(view))
    }

    /// Fetches education history.
    #[oai(path = "/education", method = "get", tag = "ApiTags::Education")]
    async fn education(
        &self,
        db_pool: Data<&SqlitePool>,
    ) -> poem::Result<Json<Vec<EducationView>>> {
        let edu = EducationRow::get_all_by_resume_id(db_pool.0, RESUME_ID)
            .await
            .map_err(poem::Error::from)?;
        let view = EducationView::from_collection(edu);
        Ok(Json(view))
    }

    /// Fetches skills, grouped by their shared skill groups.
    #[oai(path = "/skills", method = "get", tag = "ApiTags::Skills")]
    async fn skills(&self, db_pool: Data<&SqlitePool>) -> poem::Result<Json<Vec<SkillView>>> {
        let skills = SkillRow::get_all_by_resume_id(db_pool.0, RESUME_ID)
            .await
            .map_err(poem::Error::from)?;
        let view = SkillView::from_row_collection(skills);
        Ok(Json(view))
    }
}
