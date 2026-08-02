use crate::vector::{database::db_traits::db_entity::DBEntity, models::primitives::db_id::DatabaseId};

pub struct OneToMany<L, R, IDType = DatabaseId>(Vec<IDType>);

impl<L: DBEntity, R: DBEntity, IDType> From<Vec<IDType>> for OneToMany<L, R, IDType> {
    fn from(value: Vec<IDType>) -> Self {
        Self(value)
    }
}

impl Default for OneToMany {
    fn default() -> Self {
        Self(Vec::new())
    }
}
