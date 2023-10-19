use crate::api::domain::contact_info::ContactInfoRow;

#[derive(poem_openapi::Object)]
pub struct ContactInfoView {
    pub method: String,
    pub information: String,
}
impl ContactInfoView {
    pub fn new(row: ContactInfoRow) -> Self {
        Self {
            method: row.method,
            information: row.information,
        }
    }

    pub fn from_collection(rows: Vec<ContactInfoRow>) -> Vec<Self> {
        rows.into_iter().map(Self::new).collect()
    }
}
