use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum AutoTaggableType {
    #[serde(rename = "tag")]
    #[strum(to_string = "tag")]
    Tag,
    #[serde(rename = "topic")]
    #[strum(to_string = "topic")]
    Topic,
    #[serde(rename = "subject")]
    #[strum(to_string = "subject")]
    Subject,
}
