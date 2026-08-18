use crate::ecosystem::{
    db::{db::DBGuard, tables::DatabaseTable},
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use lancedb::Table;

pub async fn open_table<'a>(db: DBGuard, table: &'a DatabaseTable) -> DatabaseResult<Table> {
    db.open_table(table.to_string()).execute().await.map_err(|e| {
                                                        println!("Table Error: {:?}", e);
                                                        DatabaseError::FailToConnect
                                                    })
}
