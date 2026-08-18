use crate::{
    ai::ai_constants::{
        BASE_TEMPERATURE_CHAT, BASE_TEMPERATURE_NOTE_CREATION, BASE_TEMPERATURE_STRUCTURED_GENERATION,
        BASE_TEMPERATURE_TEXT_SUMMARIZATION,
    },
    ecosystem::{db::db_traits::db_field::DatabaseField, error_handling::db_error::DatabaseError},
};
use fake::Dummy;
use strum::IntoEnumIterator;

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
#[serde(try_from = "String", into = "String", rename_all = "kebab-case")]
pub enum AgentPrimaryTask {
    GeneralChat,
    NoteCreation,
    NoteSummarization,
    FlashCardCreation,
    FactVerification,
    BibliographyExtraction,
    VectorGenerationFromText,
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

impl AgentPrimaryTask {
    pub fn to_base_temperature(&self) -> f64 {
        match self {
            Self::GeneralChat => BASE_TEMPERATURE_CHAT as f64,
            Self::NoteCreation => BASE_TEMPERATURE_NOTE_CREATION as f64,
            Self::NoteSummarization => BASE_TEMPERATURE_TEXT_SUMMARIZATION as f64,
            Self::FlashCardCreation => BASE_TEMPERATURE_STRUCTURED_GENERATION as f64,
            Self::BibliographyExtraction => BASE_TEMPERATURE_STRUCTURED_GENERATION as f64,
            _ => 0.5,
        }
    }
}
