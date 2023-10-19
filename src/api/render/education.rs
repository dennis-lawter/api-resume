use crate::api::domain::education::EducationRow;

#[derive(poem_openapi::Object)]
pub struct EducationView {
    pub school: String,
    pub degree: String,
    pub education_start_date: String,
    pub education_end_date: String,
}
impl EducationView {
    pub fn new(row: EducationRow) -> Self {
        Self {
            school: row.school,
            degree: row.degree,
            education_start_date: row.education_start_date,
            education_end_date: row.education_end_date,
        }
    }

    pub fn from_collection(rows: Vec<EducationRow>) -> Vec<Self> {
        rows.into_iter().map(Self::new).collect()
    }
}
