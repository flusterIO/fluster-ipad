use conundrum::ecosystem::db::tables::DatabaseTable;

use crate::vector::models::primitives::db_id::DatabaseId;

pub fn fake_database_id(table: &'static str) -> DatabaseId {
    DatabaseId::new()
}
