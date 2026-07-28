use crate::ecosystem::error_handling::db_error::DatabaseResult;

/// Deprecated after the move to surreal
pub trait DatabaseFieldRepresentable<T> {
    fn to_db_representation(&self) -> T;
}

/// Deprecated after the move to surreal
pub trait TryDatabaseFieldRepresentable<T> {
    fn try_to_db_representation(&self) -> DatabaseResult<T>;
}
