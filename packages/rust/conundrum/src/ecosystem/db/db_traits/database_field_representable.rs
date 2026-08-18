use crate::ecosystem::error_handling::db_error::DatabaseResult;

pub trait DatabaseFieldRepresentable<T> {
    fn to_db_representation(&self) -> T;
}

pub trait TryDatabaseFieldRepresentable<T> {
    fn try_to_db_representation(&self) -> DatabaseResult<T>;
}
