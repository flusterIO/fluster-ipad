pub trait ActionableRequest<T> {
    async fn execute_request(&self) -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<T>;
}
