use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TaggableVariant {
    #[serde(rename = "tag")]
    Tag,
    #[serde(rename = "topic")]
    Topic,
    #[serde(rename = "subject")]
    Subject,
}
