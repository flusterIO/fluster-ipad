use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// From typescript to swift.
#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum SharedWebviewActions {
    #[serde(rename = "javascript-error")]
    #[strum(to_string = "javascript-error")]
    JavascriptError,
}

/// Events
#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum SharedWebviewEvents {
    #[serde(rename = "broadcast-local-storage")]
    #[strum(to_string = "broadcast-local-storage")]
    LocalStorageWrite,
}
