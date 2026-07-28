use serde::Deserialize;
use serde_with::SerializeDisplay;
use surrealdb::types::SurrealValue;

#[derive(SerializeDisplay, Deserialize, Default, strum_macros::Display, Clone, Debug, SurrealValue)]
pub enum SchemaVersion {
    #[serde(rename = "0.0.0")]
    #[strum(to_string = "0.0.0")]
    #[default]
    PreRelease,
}
