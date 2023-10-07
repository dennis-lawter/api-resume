use poem_openapi::payload::Json;
use poem_openapi::payload::PlainText;
use poem_openapi::ApiResponse;
use poem_openapi::Object;

use crate::domain::contact_info::ContactInfoModel;

use super::View;

#[derive(ApiResponse)]
pub enum GetContactInfoListResponse {
    #[oai(status = 200)]
    Ok(Json<ContactInfoView>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

#[derive(Object)]
pub struct ContactInfoView {
    method: String,
    information: String,
}
impl View<ContactInfoModel> for ContactInfoView {}
impl From<ContactInfoModel> for ContactInfoView {
    fn from(model: ContactInfoModel) -> Self {
        Self {
            method: model.method.clone(),
            information: model.information.clone(),
        }
    }
}
