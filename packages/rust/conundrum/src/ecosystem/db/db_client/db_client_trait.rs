use crate::ecosystem::{db::tables::DatabaseTable, error_handling::db_error::DatabaseResult};

pub trait DBClient {
    async fn clear_table(&self, tbl: DatabaseTable) -> DatabaseResult<()>;
}
