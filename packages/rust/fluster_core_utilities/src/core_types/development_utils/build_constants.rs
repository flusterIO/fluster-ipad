use serde::{Deserialize, Serialize};

#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum BuildEnvVariables {
    #[serde(rename = "FLUSTER_BUILD_ENV")]
    #[strum(to_string = "FLUSTER_BUILD_ENV")]
    /// One of "ipad' | "macos"
    BuildEnvironment,
}
