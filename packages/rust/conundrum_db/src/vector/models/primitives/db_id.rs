use std::fmt::Display;

use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;
pub use uuid::Uuid;

use conundrum::ecosystem::db::traits::database_field_representable::DatabaseFieldRepresentable;

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct DatabaseId(String);

impl Display for DatabaseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl DatabaseId {
    pub fn new() -> DatabaseId {
        DatabaseId(Uuid::new_v4().to_string())
    }
}

impl DatabaseFieldRepresentable<String> for DatabaseId {
    fn to_db_representation(&self) -> String {
        self.0.clone()
    }
}
