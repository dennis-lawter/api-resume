use crate::api::domain::contact_info::ContactInfoRow;

use super::View;

#[derive(poem_openapi::Object, Clone)]
pub struct ContactInfoView {
    pub method: String,
    pub information: String,
}

impl View<ContactInfoRow> for ContactInfoView {
    fn new(row: ContactInfoRow) -> Self {
        Self {
            method: row.method,
            information: row.information,
        }
    }

    fn from_collection(rows: Vec<ContactInfoRow>) -> Vec<Self> {
        rows.into_iter().map(Self::new).collect()
    }
}
