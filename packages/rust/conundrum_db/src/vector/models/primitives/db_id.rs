use crate::vector::models::primitives::db_id_type::DatabaseIdType;
use conundrum::ecosystem::db::tables::DatabaseTable;
use fake::{Dummy, Faker};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::fmt::Display;
use surrealdb::types::{RecordId, SurrealValue};

use crate::vector::database::db_traits::database_field::DatabaseField;

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue, Type)]
pub struct DatabaseId {
    pub table: DatabaseTable,
    pub key: DatabaseIdType,
}

impl Display for DatabaseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{:?}", self.table, self.key)
    }
}

impl DatabaseId {
    pub fn new(table: DatabaseTable) -> Self {
        DatabaseId { table: table.clone(),
                     key: DatabaseIdType::default() }
    }

    pub fn new_from_input_id(table: DatabaseTable, value: impl Into<DatabaseIdType>) -> Self {
        DatabaseId { table: table.clone(),
                     key: value.into() }
    }

    pub fn to_record_id(&self) -> RecordId {
        RecordId::new(self.table.to_string(), self.key.to_record_key())
    }
}

impl DatabaseField for DatabaseId {
    fn field_definition(field_key: &'static str, table: &conundrum::ecosystem::db::tables::DatabaseTable) -> String {
        format!("DEFINE FIELD IF NOT EXISTS {} ON {} TYPE record", field_key, table)
    }
}

impl Dummy<Faker> for DatabaseId {
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        Self::new(DatabaseTable::AutoTaggable)
    }
}
