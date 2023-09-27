// use poem_openapi::payload::Json;
// use poem_openapi::payload::PlainText;
// use poem_openapi::ApiResponse;
// use poem_openapi::Object;

// use crate::domain::experience::ExperienceDao;

// use super::View;

// #[derive(ApiResponse)]
// pub enum GetExperienceListResponse {
//     #[oai(status = 200)]
//     Ok(Json<ExperienceView>),

//     #[oai(status = 404)]
//     NotFound(PlainText<String>),
// }

// #[derive(Object)]
// pub struct ExperienceView {
//     pub employer: String,
//     pub title: String,
//     pub employment_start_date: String,
//     pub employment_end_date: String,
// }
// impl View<ExperienceDao> for ExperienceView {
//     fn from_domain(dao: &ExperienceDao) -> Self {
//         Self {
//             employer: dao.employer.clone(),
//             title: dao.title.clone(),
//             employment_start_date: dao.employment_start_date.clone(),
//             employment_end_date: dao.employment_end_date.clone(),
//         }
//     }
// }
