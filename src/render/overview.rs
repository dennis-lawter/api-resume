use poem_openapi::payload::Json;
use poem_openapi::payload::PlainText;
use poem_openapi::ApiResponse;
use poem_openapi::Object;

use crate::domain::overview::OverviewDao;

use super::View;

#[derive(ApiResponse)]
pub enum GetOverviewListResponse {
    #[oai(status = 200)]
    Ok(Json<OverviewView>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

#[derive(Object)]
pub struct OverviewView {
    pub full_name: String,
    pub title: String,
    pub objective: String,
}
impl View<OverviewDao> for OverviewView {
    fn from_domain(dao: &OverviewDao) -> Self {
        Self {
            full_name: dao.full_name.clone(),
            title: dao.title.clone(),
            objective: dao.objective.clone(),
        }
    }
}
