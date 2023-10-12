use super::View;
use crate::domain::contact_info::ContactInfoModel;
use poem_openapi::Object;

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
