use crate::ecosystem::error_handling::db_error::DatabaseError;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, strum_macros::Display, strum_macros::EnumIter)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case", into = "String", try_from = "String")]
pub enum StaticId {
    DefaultAgent,
}

impl TryFrom<String> for StaticId {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for k in <StaticId>::iter() {
            if k.to_string() == value {
                return Ok(k);
            }
        }
        return Err(DatabaseError::SerializationError);
    }
}
impl Into<String> for StaticId {
    fn into(self) -> String {
        self.to_string()
    }
}
