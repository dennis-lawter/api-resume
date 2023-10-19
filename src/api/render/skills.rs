use std::collections::HashMap;

use super::super::domain::skills::SkillRow;

#[derive(poem_openapi::Object)]
pub struct SkillView {
    pub skill_group_id: i64,
    pub group_name: String,
    pub skills: Vec<String>,
}
impl SkillView {
    pub fn from_row_collection(rows: Vec<SkillRow>) -> Vec<Self> {
        let mut map: HashMap<i64, (Self, Vec<String>)> = HashMap::new();

        for row in rows {
            let entry = map.entry(row.id).or_insert_with(|| {
                (
                    Self {
                        skill_group_id: row.id,
                        group_name: row.group_name.clone(),
                        skills: Vec::new(),
                    },
                    Vec::new(),
                )
            });

            entry.1.push(row.skill_name);
        }

        map.into_iter()
            .map(|(_, (mut view, skills))| {
                view.skills = skills;
                view
            })
            .collect()
    }
}
