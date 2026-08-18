use crate::ecosystem::error_handling::db_error::DatabaseResult;

pub trait ValidateSelf {
    async fn validate(&self) -> DatabaseResult<()>;
}
