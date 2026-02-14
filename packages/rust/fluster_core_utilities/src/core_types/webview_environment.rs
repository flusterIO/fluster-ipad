use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// From typescript to swift.
#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum WebviewEnvironment {
    #[serde(rename = "fluster-mac")]
    #[strum(to_string = "fluster-mac")]
    MacOS,
    #[serde(rename = "fluster-ipad")]
    #[strum(to_string = "fluster-ipad")]
    IPad,
    #[serde(rename = "fluster-multi-platform-desktop")]
    #[strum(to_string = "fluster-multi-platform-desktop")]
    MultiPlatformDesktop,
}
