use serde::{Deserialize, Serialize};

#[typeshare::typeshare]
#[derive(Clone, Debug, Serialize, Default, Deserialize, strum_macros::Display)]
pub enum MermaidTheme {
    #[serde(rename = "default")]
    #[strum(to_string = "default")]
    #[default]
    Default,
}
