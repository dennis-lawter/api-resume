use std::collections::HashMap;

use super::super::domain::experience::ExperienceRow;

#[derive(poem_openapi::Object)]
pub struct ExperienceView {
    pub employer: String,
    pub title: String,
    pub employment_start_date: String,
    pub employment_end_date: String,
    pub achievements: Vec<String>,
}
impl ExperienceView {
    pub fn from_row_collection(rows: Vec<ExperienceRow>) -> Vec<Self> {
        let mut map: HashMap<i64, (Self, Vec<String>)> = HashMap::new();

        for row in rows {
            let entry = map.entry(row.id).or_insert_with(|| {
                (
                    Self {
                        employer: row.employer.clone(),
                        title: row.title.clone(),
                        employment_start_date: row.employment_start_date.clone(),
                        employment_end_date: row.employment_end_date.clone(),
                        achievements: Vec::new(),
                    },
                    Vec::new(),
                )
            });

            entry.1.push(row.achievement);
        }

        map.into_iter()
            .map(|(_, (mut view, achievements))| {
                view.achievements = achievements;
                view
            })
            .collect()
    }
}
