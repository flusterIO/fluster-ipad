use conundrum::ecosystem::error_handling::db_error::DatabaseResult;

pub trait PureModelInstanceMethods: Send + Sync {
    fn save_self(&self) -> DatabaseResult<()>;
}
