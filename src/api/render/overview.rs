use crate::api::domain::overview::OverviewRow;

use super::contact_info::ContactInfoView;
use super::education::EducationView;
use super::experience::ExperienceView;
use super::skills::SkillView;
use super::View;

#[derive(poem_openapi::Object, Clone)]
pub struct OverviewView {
    pub full_name: String,
    pub title: String,
    pub objective: String,
    pub contact_info: Vec<ContactInfoView>,
    pub skills: Vec<SkillView>,
    pub experience: Vec<ExperienceView>,
    pub education: Vec<EducationView>,
}
impl View<OverviewRow> for OverviewView {
    fn new(row: OverviewRow) -> Self {
        Self {
            full_name: row.full_name,
            title: row.title,
            objective: row.objective,
            contact_info: vec![],
            skills: vec![],
            experience: vec![],
            education: vec![],
        }
    }

    fn from_collection(rows: Vec<OverviewRow>) -> Vec<Self> {
        rows.into_iter().map(Self::new).collect()
    }
}
