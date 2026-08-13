use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use strum::IntoEnumIterator;

#[derive(serde::Serialize,
           serde::Deserialize,
           Clone,
           Debug,
           specta::Type,
           fake::Dummy,
           strum_macros::Display,
           strum_macros::EnumIter)]
#[serde(try_from = "String", into = "String", rename_all = "snake_case")]
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
