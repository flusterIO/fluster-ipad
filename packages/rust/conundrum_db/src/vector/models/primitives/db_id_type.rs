use std::fmt::Display;

use serde::{Deserialize, Serialize};
use specta::Type;
use surrealdb::types::SurrealValue;
use surrealdb_types::RecordIdKey;

#[derive(Serialize, Deserialize, SurrealValue, Clone, Debug, Type)]
pub enum DatabaseIdType {
    Int(i64),
    String(String),
}

impl Default for DatabaseIdType {
    fn default() -> Self {
        DatabaseIdType::String(uuid::Uuid::new_v4().to_string())
    }
}

impl DatabaseIdType {
    pub fn to_record_key(&self) -> RecordIdKey {
        match self {
            DatabaseIdType::Int(n) => RecordIdKey::Number(*n),
            DatabaseIdType::String(s) => RecordIdKey::String(s.clone()),
        }
    }
}

impl Display for DatabaseIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", match self {
            DatabaseIdType::String(s) => s.clone(),
            DatabaseIdType::Int(n) => n.to_string(),
        })
    }
}
