use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct AiPhase2Response {
    pub success: bool,
    pub replace_with: Option<String>,
    /// An optional message that will be displayed to the user in a toast if
    /// present.
    pub user_message: Option<String>,
}
