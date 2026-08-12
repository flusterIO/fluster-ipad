use crate::errors::server_error::{ServerError, ServerResult};

pub trait ActionableTool<ResultType> {
    async fn execute() -> ServerResult<ResultType>;
}
