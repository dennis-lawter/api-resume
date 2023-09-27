// use poem::error::NotFoundError;
// use poem::web::Data;
// use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use poem_openapi::Tags;
// use sqlx::SqlitePool;

// use crate::domain::contact_info::ContactInfoDao;
// use crate::domain::experience::ExperienceDao;
// use crate::domain::overview::OverviewDao;
// use crate::domain::Dao;
// use crate::render::contact_info::ContactInfoView;
// use crate::render::experience::ExperienceView;
// use crate::render::overview::OverviewView;
// use crate::render::View;

pub struct ApiV1;

#[allow(dead_code)]
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

    // /// Resume Overview
    // #[oai(path = "/", method = "get", tag = "ApiTags::Info")]
    // async fn overview(&self, pool: Data<&SqlitePool>) -> poem::Result<Json<Vec<OverviewView>>> {
    //     let overviews = OverviewDao::retrieve_all(pool.0)
    //         .await
    //         .map_err(|_| NotFoundError)?;

    //     let overview_views: Vec<OverviewView> = overviews
    //         .into_iter()
    //         .map(|dao| OverviewView::from_domain(&dao))
    //         .collect();

    //     Ok(Json(overview_views))
    // }

    // /// Contact Information
    // #[oai(path = "/contact", method = "get", tag = "ApiTags::Info")]
    // async fn contact_all(
    //     &self,
    //     pool: Data<&SqlitePool>,
    // ) -> poem::Result<Json<Vec<ContactInfoView>>> {
    //     let contact_infos = ContactInfoDao::retrieve_all(pool.0)
    //         .await
    //         .map_err(|_| NotFoundError)?;

    //     let contact_info_views: Vec<ContactInfoView> = contact_infos
    //         .into_iter()
    //         .map(|dao| ContactInfoView::from_domain(&dao))
    //         .collect();

    //     Ok(Json(contact_info_views))
    // }

    // /// Experience
    // #[oai(path = "/experience", method = "get", tag = "ApiTags::Experience")]
    // async fn experience_all(
    //     &self,
    //     pool: Data<&SqlitePool>,
    // ) -> poem::Result<Json<Vec<ExperienceView>>> {
    //     let experiences = ExperienceDao::retrieve_all(pool.0)
    //         .await
    //         .map_err(|_| NotFoundError)?;

    //     let experience_views: Vec<ExperienceView> = experiences
    //         .into_iter()
    //         .map(|dao| ExperienceView::from_domain(&dao))
    //         .collect();

    //     Ok(Json(experience_views))
    // }
}
