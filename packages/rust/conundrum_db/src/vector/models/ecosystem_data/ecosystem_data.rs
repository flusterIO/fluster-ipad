use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

use crate::vector::{database::schema_version::schema_version::SchemaVersion, models::date_time::date_time::DateTime};

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct EcosystemData {
    /// TODO: Do something... like literally anything for security here. Some
    /// sort of ceritificate will be required in the future, but this will
    /// work while nobody's building for this anyways...
    pub application_white_list: Vec<String>,
    pub schema_version: SchemaVersion,
    pub initialized_on: DateTime,
    pub last_sync: DateTime,
}
