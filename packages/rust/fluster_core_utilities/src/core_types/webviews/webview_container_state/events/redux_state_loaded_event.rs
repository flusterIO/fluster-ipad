use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct ReduxStateLoadedEvent {
    /// The currently focused note_id.
    pub note_id: Option<String>,
}
