use fake::Dummy;
use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

use crate::vector::database::db_traits::database_field::DatabaseField;

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue, Dummy)]
pub enum AIGeneratedStatus {
    /// This was completely written by humans.
    None = 0,
    /// This is primarily human written, but with some AI generated content.
    Some = 1,
    /// Mostly AI written with some human input
    Most = 2,
    /// Completely AI written.
    All = 3,
}

impl DatabaseField for AIGeneratedStatus {
    fn field_definition(field_key: &'static str, table: &conundrum::ecosystem::db::tables::DatabaseTable) -> String {
        format!("DEFINE FIELD IF NOT EXISTS {} ON {} TYPE int;", field_key, table)
    }
}
