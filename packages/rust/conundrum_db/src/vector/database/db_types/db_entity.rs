use std::sync::Arc;

use conundrum::ecosystem::{db::tables::DatabaseTable, error_handling::db_error::DatabaseResult};

use crate::vector::models::primitives::db_id::DatabaseId;

pub trait DBEntity<PrimaryKeyType = DatabaseId>: Sized {
    fn table() -> DatabaseTable;
}
