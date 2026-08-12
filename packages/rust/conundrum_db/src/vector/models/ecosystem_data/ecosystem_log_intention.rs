use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use strum::IntoEnumIterator;

#[derive(serde::Serialize,
           serde::Deserialize,
           Clone,
           Debug,
           specta::Type,
           strum_macros::EnumIter,
           strum_macros::EnumString,
           strum_macros::Display)]
#[strum(serialize_all = "kebab-case")]
#[serde(try_from = "String")]
pub enum EcosystemLogIntention {
    LogError,
    LogWarning,
    LogInformation,
    GitStatusChange,
    ProcessComplete,
}

impl TryFrom<String> for EcosystemLogIntention {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for k in EcosystemLogIntention::iter() {
            if k.to_string() == value {
                return Ok(k);
            }
        }
        return Err(DatabaseError::SerializationError);
    }
}
