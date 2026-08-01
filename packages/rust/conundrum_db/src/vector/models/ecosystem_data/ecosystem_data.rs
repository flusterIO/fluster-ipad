use serde::{Deserialize, Serialize};
use specta::Type;

use crate::vector::{
    database::schema_version::{schema_version::SchemaVersion, server_version::ServerVersion},
    models::{date_time::date_time::DateTime, ecosystem_data::ecosystem_settings::EcosystemSettings},
};

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct VersionData {
    pub database: SchemaVersion,
    pub server: ServerVersion,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EcosystemData {
    /// TODO: Do something... like literally anything for security here. Some
    /// sort of ceritificate will be required in the future, but this will
    /// work while nobody's building for this anyways...
    pub application_white_list: Vec<String>,
    /// Indicates whether the onboarding flow has been shown for the REST api.
    pub have_shown_api_onboarding: bool,
    pub schema_version: SchemaVersion,
    pub initialized_on: DateTime,
    pub last_sync: Option<DateTime>,
    pub settings: EcosystemSettings,
}

impl Default for EcosystemData {
    fn default() -> Self {
        Self { application_white_list: Default::default(),
               have_shown_api_onboarding: false,
               schema_version: SchemaVersion::current_version(),
               initialized_on: DateTime::new_now(),
               last_sync: None,
               settings: Default::default() }
    }
}
