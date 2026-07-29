use std::fmt::Display;

use conundrum::ecosystem::db::tables::DatabaseTable;
use fake::{Dummy, Faker};
use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;
use surrealdb_types::{RecordId, RecordIdKey, uuid};

use crate::vector::database::db_traits::database_field::DatabaseField;

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct DatabaseId(pub RecordId);

impl Display for DatabaseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{:?}", self.0.table, self.0.key)
    }
}

impl DatabaseId {
    pub fn new(table: DatabaseTable) -> Self {
        DatabaseId(RecordId::new(table.to_string(), uuid::Uuid::new_v4().to_string()))
    }

    pub fn new_from_input_id(table: DatabaseTable, value: impl Into<RecordIdKey>) -> Self {
        DatabaseId(RecordId::new(table.to_string(), value))
    }
}

impl DatabaseField for DatabaseId {
    fn field_definition(field_key: &'static str, table: &conundrum::ecosystem::db::tables::DatabaseTable) -> String {
        format!("DEFINE FIELD IF NOT EXISTS {} ON {} TYPE string;", field_key, table)
    }
}

impl Dummy<Faker> for DatabaseId {
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        Self::new(DatabaseTable::AutoTaggable)
    }
}
