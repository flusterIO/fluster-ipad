use crate::lifted_models::primitives::db_id::DatabaseId;

pub fn fake_database_id(table: &'static str) -> DatabaseId {
    DatabaseId::new()
}
