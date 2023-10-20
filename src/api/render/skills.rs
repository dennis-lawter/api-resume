use std::collections::HashMap;

use super::super::domain::skills::SkillRow;
use super::View;

#[derive(poem_openapi::Object)]
pub struct SkillView {
    pub group_name: String,
    pub skills: Vec<String>,
}
impl View<SkillRow> for SkillView {
    fn new(row: SkillRow) -> Self {
        Self {
            group_name: row.group_name,
            skills: vec![],
        }
    }
    fn from_collection(rows: Vec<SkillRow>) -> Vec<Self> {
        let mut map: HashMap<i64, (Self, Vec<String>)> = HashMap::new();

        for row in rows {
            let entry = map.entry(row.id).or_insert_with(|| {
                (
                    Self {
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
