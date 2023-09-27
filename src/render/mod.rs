use crate::domain::StaticModel;

pub mod contact_info;
pub mod experience;
pub mod overview;

pub trait View<M>: From<M>
where
    M: StaticModel,
{
}
