use poem_openapi::payload::Json;
use poem_openapi::payload::PlainText;
use poem_openapi::ApiResponse;
use poem_openapi::Object;

use crate::domain::contact_info::ContactInfoDao;

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
impl View<ContactInfoDao> for ContactInfoView {
    fn from_domain(dao: &ContactInfoDao) -> Self {
        Self {
            method: dao.method.clone(),
            information: dao.information.clone(),
        }
    }
}
