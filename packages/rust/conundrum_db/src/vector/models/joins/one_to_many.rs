use crate::vector::models::primitives::db_id::DatabaseId;

pub struct OneToMany<IDType = DatabaseId>(Vec<IDType>);

impl<IDType> From<Vec<IDType>> for OneToMany<IDType> {
    fn from(value: Vec<IDType>) -> Self {
        Self(value)
    }
}

impl<T> Default for OneToMany<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}
