use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, strum_macros::Display, Dummy)]
pub enum TaggableVariant {
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
