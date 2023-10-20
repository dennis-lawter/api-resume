use super::domain::DomainRow;

pub mod contact_info;
pub mod education;
pub mod experience;
pub mod overview;
pub mod skills;

pub trait View<R: DomainRow> {
    fn new(row: R) -> Self;
    fn from_collection(rows: Vec<R>) -> Vec<Self>
    where
        Self: Sized + Send + 'static;
}
