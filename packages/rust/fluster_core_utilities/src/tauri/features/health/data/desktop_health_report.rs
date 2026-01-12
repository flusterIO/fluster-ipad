use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Type, Serialize, Deserialize, Clone)]
pub struct DesktopHealthReport {
    pub database_tables_exist: bool,
    /// This boolean describes the overall health of the desktop app. If any inidividual field
    /// that warrents re-initializing is false, this field will be false.
    pub healthy: bool,
}
