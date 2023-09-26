pub mod contact_info;
pub mod experience;
pub mod overview;

use crate::domain::Dao;

pub trait View<D: Dao> {
    fn from_domain(dao: &D) -> Self;
}
