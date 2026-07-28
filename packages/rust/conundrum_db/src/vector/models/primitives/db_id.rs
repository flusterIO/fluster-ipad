use std::fmt::Display;

use fake::{Dummy, Faker};
use serde::{Deserialize, Serialize};
use surrealdb::types::{SurrealValue, Uuid};

use conundrum::ecosystem::db::traits::database_field_representable::DatabaseFieldRepresentable;

use crate::vector::database::db_traits::database_field::DatabaseField;

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct DatabaseId(Uuid);

impl Dummy<Faker> for DatabaseId {
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        DatabaseId::default()
    }
}

impl Display for DatabaseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for DatabaseId {
    fn default() -> Self {
        DatabaseId(Uuid::new())
    }
}

impl DatabaseFieldRepresentable<Uuid> for DatabaseId {
    fn to_db_representation(&self) -> Uuid {
        self.0.clone()
    }
}

impl DatabaseField for DatabaseId {
    fn field_definition(field_key: &'static str, table: &conundrum::ecosystem::db::tables::DatabaseTable) -> String {
        format!("DEFINE FIELD IF NOT EXISTS {} ON {} TYPE uuid;", field_key, table)
    }
}
