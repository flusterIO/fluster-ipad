use serde::Deserialize;
use serde_with::SerializeDisplay;
use specta::Type;
use surrealdb::types::SurrealValue;

#[derive(SerializeDisplay, Deserialize, Default, strum_macros::Display, Clone, Debug, SurrealValue, Type)]
pub enum ServerVersion {
    #[serde(rename = "0.0.0")]
    #[strum(to_string = "0.0.0")]
    #[default]
    PreRelease,
}

impl ServerVersion {
    pub fn current_version() -> Self {
        Self::PreRelease
    }
}
