pub trait ActionableRequest<T> {
    async fn execute_request(&self) -> crate::ecosystem::error_handling::db_error::DatabaseResult<T>;
}
