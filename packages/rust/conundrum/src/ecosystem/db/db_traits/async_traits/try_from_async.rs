use crate::ecosystem::error_handling::db_error::DatabaseResult;

pub trait TryFromAsync<T, ResultType = Self> {
    async fn try_from_async(input: T) -> DatabaseResult<Self>
        where Self: Sized;
}

pub trait FromAsync<T, ResultType = Self> {
    async fn from_async(input: T) -> Self
        where Self: Sized;
}
