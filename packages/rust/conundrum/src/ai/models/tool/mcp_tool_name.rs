use strum::IntoEnumIterator;

use crate::ecosystem::{db::db_traits::db_field::DatabaseField, error_handling::db_error::DatabaseError};

#[derive(serde::Serialize,
           serde::Deserialize,
           Clone,
           Debug,
           specta::Type,
           fake::Dummy,
           strum_macros::Display,
           strum_macros::EnumIter)]
#[strum(serialize_all = "snake_case")]
#[serde(try_from = "String", into = "String", rename_all = "snake_case")]
#[typeshare::typeshare]
pub enum MCPToolName {
    HelloWorld,
    QueryWorkspaces,
}

impl TryFrom<String> for MCPToolName {
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

impl From<MCPToolName> for String {
    fn from(value: MCPToolName) -> Self {
        value.to_string()
    }
}

impl DatabaseField for MCPToolName {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}
