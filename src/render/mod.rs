use crate::domain::Dao;

pub mod contact_info;
pub mod overview;

pub trait View<D: Dao> {
    fn from_domain(dao: &D) -> Self;
}
