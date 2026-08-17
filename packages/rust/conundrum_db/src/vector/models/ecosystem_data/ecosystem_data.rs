use conundrum::lifted_models::primitives::date_time::DateTime;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::vector::{
    database::schema_version::{schema_version::SchemaVersion, server_version::ServerVersion},
    models::ecosystem_data::onboarding_dialogs::OnboardingDialogs,
};

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct VersionData {
    pub database: SchemaVersion,
    pub server: ServerVersion,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EcosystemData {
    pub schema_version: SchemaVersion,
    pub initialized_on: DateTime,
    pub last_sync: Option<DateTime>,
    pub onboarding_dialogs: OnboardingDialogs,
}

impl Default for EcosystemData {
    fn default() -> Self {
        Self { schema_version: SchemaVersion::current_version(),
               initialized_on: DateTime::new_now(),
               last_sync: None,
               onboarding_dialogs: OnboardingDialogs::default() }
    }
}
