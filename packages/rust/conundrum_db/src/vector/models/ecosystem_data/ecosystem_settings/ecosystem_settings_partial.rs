use fake::Dummy;
use serde::{Deserialize, Serialize};
use serde_with::DeserializeFromStr;

use crate::vector::models::primitives::db_id_single_instance::DBIDSingleInstance;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct EcosystemSettingsPartial {
    pub id: DBIDSingleInstance,
    /// The number of days that logs should be preserved. Default is 7.
    pub save_log_duration: Option<u16>,
    pub vectorize_logs: Option<bool>,
}
