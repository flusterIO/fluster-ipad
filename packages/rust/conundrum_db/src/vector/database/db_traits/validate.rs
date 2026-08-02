use conundrum::ecosystem::error_handling::db_error::DatabaseError;

pub trait ValidateSelf {
    fn validate(&self) -> DatabaseError<()>;
}
