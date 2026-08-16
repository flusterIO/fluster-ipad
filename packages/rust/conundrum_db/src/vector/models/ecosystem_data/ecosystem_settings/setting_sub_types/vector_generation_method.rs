use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use fake::Dummy;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Serialize,
           Deserialize,
           strum_macros::Display,
           strum_macros::EnumIter,
           strum_macros::EnumString,
           Debug,
           Dummy,
           specta::Type,
           Clone)]
#[strum(serialize_all = "kebab-case")]
pub enum VectorGenerationMethod {
    /// Allow the framework to update vectors as the content changes. This will
    /// keep your app most up to date at the expense of more token
    /// expenditure.
    Automatic,
    /// Only update vectors when the user explicitly requests to update vectors.
    OnUserSentRequest,
}

impl TryFrom<String> for VectorGenerationMethod {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for k in VectorGenerationMethod::iter() {
            if k.to_string() == value {
                return Ok(k);
            }
        }
        log::warn!("Failed to deserialize a VectorGenerationMethod enum variant.");
        Err(DatabaseError::SerializationError)
    }
}
