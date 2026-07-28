use conundrum::ecosystem::db::traits::database_field_representable::DatabaseFieldRepresentable;
use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
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

impl<'a> DatabaseFieldRepresentable<&'a str> for AIGeneratedStatus {
    fn to_db_representation(&self) -> &'a str {
        match self {
            Self::None => "none",
            Self::Some => "some",
            Self::Most => "most",
            Self::All => "all",
        }
    }
}
