use conundrum::ecosystem::db::db_traits::{db_field::DatabaseField, db_identifiable::DatabaseIdentifiable};
use fake::Dummy;
use serde::{Deserialize, Serialize};

/// # TODO
/// Definitely something for version 2, but eventually keyboard settings will be
/// modifiable and sharable across the ecosystem, so you can swap out
/// applications with as little hickup as possible.
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

impl DatabaseIdentifiable for EcosystemApplicationAction {
    fn to_predicate(&self, field_key: &str) -> String {
        format!("{} = \"{}\"", field_key, self.to_string())
    }
}
