use crate::domain::overview::OverviewModel;
use poem_openapi::Object;

use super::contact_info::ContactInfoView;
use super::View;

#[derive(Object)]
pub struct OverviewView {
    pub full_name: String,
    pub title: String,
    pub objective: String,
    pub contact_info: Vec<ContactInfoView>,
}

impl View<OverviewModel> for OverviewView {}

impl From<OverviewModel> for OverviewView {
    fn from(model: OverviewModel) -> Self {
        let ci_views = model
            .contact_infos
            .into_iter()
            .map(|boxed_ci| ContactInfoView::from(*boxed_ci))
            .collect();
        Self {
            full_name: model.full_name,
            title: model.title,
            objective: model.objective,
            contact_info: ci_views,
        }
    }
}
