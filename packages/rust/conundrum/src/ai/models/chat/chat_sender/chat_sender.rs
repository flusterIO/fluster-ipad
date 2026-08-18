use crate::ecosystem::{db::db_traits::db_field::DatabaseField, error_handling::db_error::DatabaseError};
use fake::Dummy;
use strum::IntoEnumIterator;

#[derive(serde::Serialize,
           serde::Deserialize,
           Clone,
           Debug,
           strum_macros::Display,
           strum_macros::EnumIter,
           Dummy,
           specta::Type)]
#[serde(rename_all = "snake_case", try_from = "String", into = "String")]
pub enum ChatParticipant {
    User,
    AI,
    SystemPrompt,
}

impl Into<String> for ChatParticipant {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<String> for ChatParticipant {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for k in Self::iter() {
            if k.to_string() == value {
                return Ok(k);
            }
        }
        Err(DatabaseError::SerializationError)
    }
}

impl DatabaseField for ChatParticipant {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}
