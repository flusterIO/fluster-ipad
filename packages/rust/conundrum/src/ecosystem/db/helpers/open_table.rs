use crate::ecosystem::{
    db::{db::DBGuard, tables::DatabaseTable},
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use lancedb::Table;

pub async fn open_table(db: DBGuard, table: DatabaseTable) -> DatabaseResult<Table> {
    db.open_table(table.to_string()).execute().await.map_err(|e| {
                                                        println!("Table Error: {:?}", e);
                                                        DatabaseError::FailToConnect
                                                    })
}
