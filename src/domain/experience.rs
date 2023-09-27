// use async_trait::async_trait;
// use color_eyre::eyre::Result;
// use sqlx::SqlitePool;

// use super::Dao;

// #[derive(sqlx::FromRow)]
// pub struct ExperienceDao {
//     pub id: i64,
//     pub employer: String,
//     pub title: String,
//     pub employment_start_date: String,
//     pub employment_end_date: String,
// }

// #[async_trait]
// impl Dao for ExperienceDao {
//     async fn retrieve_all(pool: &SqlitePool) -> Result<Vec<Box<ExperienceDao>>> {
//         let contact_infos_result: Result<Vec<ExperienceDao>, sqlx::Error> = sqlx::query_as!(
//             ExperienceDao,
//             "select
//                 id,
//                 employer,
//                 title,
//                 employment_start_date,
//                 employment_end_date
//             from
//                 previous_position
//             order by
//                 id asc"
//         )
//         .fetch_all(pool)
//         .await;

//         match contact_infos_result {
//             Ok(contact_infos) => {
//                 let boxed_contact_infos: Vec<Box<ExperienceDao>> =
//                     contact_infos.into_iter().map(Box::new).collect();

//                 Ok(boxed_contact_infos)
//             }
//             Err(err) => Err(err.into()),
//         }
//     }
// }
