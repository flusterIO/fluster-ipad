use conundrum::ecosystem::{
    db::tables::DatabaseTable,
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use surrealdb_types::RecordId;

use crate::vector::database::db::ArcMutexDB;

pub trait PureModelStaticMethods {
    fn table() -> DatabaseTable;

    async fn delete_by_id(db: &ArcMutexDB, id: &str) -> DatabaseResult<()> {
        let tbl = Self::table();
        let locked_db = db.clone().lock_owned().await;
        let r: Option<RecordId> = locked_db.delete((tbl.to_string(), id))
                                           .await
                                           .map_err(|e| DatabaseError::DatabaseError { source: Some(e) })?;
        drop(locked_db);
        Ok(())
    }
    fn schema() -> String;
    /// Returns an optional string that will be ran during initialization to set
    /// any indices on the table associated with Self.
    fn db_index_definitions() -> Option<String> {
        None
    }
    /// Returns the strings defining the schema of the relationships used by
    /// this model.
    fn relation_definitions() -> Option<String> {
        None
    }
}
