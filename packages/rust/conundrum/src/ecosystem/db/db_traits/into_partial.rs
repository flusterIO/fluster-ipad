use crate::ecosystem::error_handling::db_error::DatabaseError;

pub trait IntoPartial<PartialType> {
    fn into_partial(&self) -> PartialType;
}
