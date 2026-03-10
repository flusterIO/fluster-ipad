use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct EditorBannerNotification {
    pub title: String,
    pub id: String,
    pub body: Option<String>,
    pub timeout: Option<u32>,
}
