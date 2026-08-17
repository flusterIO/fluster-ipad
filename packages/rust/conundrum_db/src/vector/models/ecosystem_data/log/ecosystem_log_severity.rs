use conundrum::ecosystem::{db::db_traits::db_field::DatabaseField, error_handling::db_error::DatabaseError};
use fake::Dummy;
use strum::IntoEnumIterator;

#[derive(serde::Serialize,
           serde::Deserialize,
           Clone,
           Debug,
           specta::Type,
           strum_macros::EnumIter,
           strum_macros::EnumString,
           strum_macros::Display,
           Dummy)]
#[strum(serialize_all = "kebab-case")]
#[serde(try_from = "String", into = "String", rename_all = "kebab-case")]
pub enum EcosystemLogSeverity {
    Success,
    Information,
    Warning,
    Error,
}

impl From<EcosystemLogSeverity> for String {
    fn from(value: EcosystemLogSeverity) -> Self {
        value.to_string()
    }
}

impl TryFrom<String> for EcosystemLogSeverity {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, <EcosystemLogSeverity as TryFrom<String>>::Error> {
        for k in EcosystemLogSeverity::iter() {
            if k.to_string() == value {
                return Ok(k);
            }
        }
        Err(DatabaseError::SerializationError)
    }
}

impl DatabaseField for EcosystemLogSeverity {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}
