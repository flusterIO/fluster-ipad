use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use fake::Dummy;
use strum::IntoEnumIterator;

use crate::vector::database::db_traits::db_field::DatabaseField;

#[derive(serde::Serialize,
           serde::Deserialize,
           Clone,
           Debug,
           specta::Type,
           strum_macros::EnumIter,
           strum_macros::EnumString,
           strum_macros::Display,
           Dummy)]
pub enum EcosystemLogIntention {
    #[serde(rename = "git-status-change")]
    #[strum(to_string = "git-status-change")]
    GitStatusChange,
    #[serde(rename = "process-complete")]
    #[strum(to_string = "process-complete")]
    ProcessComplete,
    #[serde(rename = "entity-created")]
    #[strum(to_string = "entity-created")]
    EntityCreated,
    #[serde(rename = "entity-updated")]
    #[strum(to_string = "entity-updated")]
    EntityUpdated,
    #[serde(rename = "entity-deleted")]
    #[strum(to_string = "entity-deleted")]
    EntityDeleted,
}

impl From<EcosystemLogIntention> for String {
    fn from(value: EcosystemLogIntention) -> Self {
        value.to_string()
    }
}

impl TryFrom<String> for EcosystemLogIntention {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for k in EcosystemLogIntention::iter() {
            if k.to_string() == value {
                return Ok(k);
            }
        }
        Err(DatabaseError::SerializationError)
    }
}

impl DatabaseField for EcosystemLogIntention {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}
