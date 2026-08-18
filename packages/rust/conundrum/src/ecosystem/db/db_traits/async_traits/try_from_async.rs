use crate::ecosystem::error_handling::db_error::DatabaseResult;

pub trait TryFromAsync<T, ResultType = Self> {
    async fn try_from_async(input: T) -> DatabaseResult<Self>
        where Self: Sized;
}
