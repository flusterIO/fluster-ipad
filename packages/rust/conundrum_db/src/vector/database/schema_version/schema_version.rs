use serde::Deserialize;
use serde_with::SerializeDisplay;
use specta::Type;

#[derive(SerializeDisplay, Deserialize, Default, strum_macros::Display, Clone, Debug, Type)]
pub enum SchemaVersion {
    #[serde(rename = "0.0.0")]
    #[strum(to_string = "0.0.0")]
    #[default]
    PreRelease,
}

impl SchemaVersion {
    pub fn current_version() -> SchemaVersion {
        SchemaVersion::PreRelease
    }
}
