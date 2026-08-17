use conundrum::ecosystem::db::db_traits::db_field::DatabaseField;
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, strum_macros::Display)]
pub enum EcosystemApplicationAction {
    #[serde(rename = "toggle-command-palette")]
    #[strum(to_string = "toggle-command-palette")]
    ToggleCommandPalette,
    #[serde(rename = "toggle-side-panel")]
    #[strum(to_string = "toggle-side-panel")]
    ToggleSidePanel,
}

impl DatabaseField for EcosystemApplicationAction {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}
