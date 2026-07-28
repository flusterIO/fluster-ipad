use std::fmt::Display;

use serde::{Deserialize, Serialize};
use surrealdb::types::{SurrealValue, Uuid};

use conundrum::ecosystem::db::traits::database_field_representable::DatabaseFieldRepresentable;

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct DatabaseId(Uuid);

impl Display for DatabaseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl DatabaseId {
    pub fn new() -> DatabaseId {
        DatabaseId(Uuid::new())
    }
}

impl DatabaseFieldRepresentable<Uuid> for DatabaseId {
    fn to_db_representation(&self) -> Uuid {
        self.0.clone()
    }
}
