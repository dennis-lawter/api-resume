use poem_openapi::payload::Json;
use poem_openapi::payload::PlainText;
use poem_openapi::ApiResponse;
use poem_openapi::Object;

use crate::domain::overview::OverviewModel;

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
impl View<OverviewModel> for OverviewView {}
impl From<OverviewModel> for OverviewView {
    fn from(model: OverviewModel) -> Self {
        Self {
            full_name: model.full_name.clone(),
            title: model.title.clone(),
            objective: model.objective.clone(),
        }
    }
}
