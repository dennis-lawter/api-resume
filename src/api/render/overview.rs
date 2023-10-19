use crate::api::domain::overview::OverviewRow;

use super::contact_info::ContactInfoView;
use super::education::EducationView;
use super::experience::ExperienceView;
use super::skills::SkillView;

#[derive(poem_openapi::Object)]
pub struct OverviewView {
    pub full_name: String,
    pub title: String,
    pub objective: String,
    pub contact_info: Vec<ContactInfoView>,
    pub skills: Vec<SkillView>,
    pub experience: Vec<ExperienceView>,
    pub education: Vec<EducationView>,
}
impl OverviewView {
    pub fn new(
        overview_row: OverviewRow,
        contact_info: Vec<ContactInfoView>,
        skills: Vec<SkillView>,
        experience: Vec<ExperienceView>,
        education: Vec<EducationView>,
    ) -> Self {
        Self {
            full_name: overview_row.full_name,
            title: overview_row.title,
            objective: overview_row.objective,
            contact_info,
            skills,
            experience,
            education,
        }
    }
}
