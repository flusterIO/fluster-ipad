use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use fake::Dummy;
use strum::IntoEnumIterator;

use crate::vector::database::db_traits::db_field::DatabaseField;

#[derive(serde::Serialize,
           serde::Deserialize,
           Clone,
           Debug,
           specta::Type,
           Dummy,
           strum_macros::EnumIter,
           strum_macros::EnumString,
           strum_macros::Display)]
#[strum(serialize_all = "kebab-case")]
#[serde(try_from = "String", into = "String")]
pub enum AgentPrimaryTask {
    NoteCreation,
    NoteSummarization,
    FlashCardCreation,
    FactVerification,
    BibliographyExtraction,
    VectorGeneration,
}

impl TryFrom<String> for AgentPrimaryTask {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for k in AgentPrimaryTask::iter() {
            if k.to_string() == value {
                return Ok(k);
            }
        }
        return Err(DatabaseError::SerializationError);
    }
}

impl Into<String> for AgentPrimaryTask {
    fn into(self) -> String {
        self.to_string()
    }
}

impl DatabaseField for AgentPrimaryTask {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}
