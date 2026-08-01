use std::sync::Arc;

use conundrum::ecosystem::db::tables::DatabaseTable;
use lancedb::arrow::arrow_schema::Schema;

/// An entity refers to an object *exactly* as it appears in the database, or as
/// close to that as we can get with Rust types. If you're looking for something
/// more composed and usable, look into the equivalent 'Model'.
pub trait DBEntity {
    fn arrow_schema() -> Arc<Schema>;
    fn table() -> DatabaseTable;
}
