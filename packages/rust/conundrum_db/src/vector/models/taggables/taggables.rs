use crate::vector::models::taggables::{subject::Subject, tag_list::TagList, topic::Topic};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Taggables {
    pub tags: TagList,
    pub topic: Option<Topic>,
    pub subject: Option<Subject>,
}
