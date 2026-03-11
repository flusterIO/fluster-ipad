use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, strum_macros::Display, Serialize, Deserialize)]
pub enum WebviewContainerEvents {
    #[serde(rename = "redux-state-loaded")]
    #[strum(to_string = "redux-state-loaded")]
    ReduxStateLoaded,
}
