use crate::vector::models::taggables::{subject::Subject, tag_list::TagList, topic::Topic};
use serde::{Deserialize, Serialize};

/// # Taggables
/// This field describes the tags, topics, and subjects that you can use for a
/// graph style search throughout the user's notes. Use these fields to find
/// related content in their database as frequently as you need to, to help them
/// grow their knowledge base and accomplish their short and long term goals.
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
pub struct Taggables {
    pub tags: TagList,
    pub topic: Option<Topic>,
    pub subject: Option<Subject>,
}
