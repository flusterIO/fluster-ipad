use conundrum::ecosystem::db::tables::DatabaseTable;

use crate::vector::models::primitives::db_id::DatabaseId;

pub fn fake_database_id(table: &'static str) -> DatabaseId {
    let s =
        DatabaseTable::try_from(table.to_string()).expect("Must always convert a static string to a database table");
    DatabaseId::new(s)
}
