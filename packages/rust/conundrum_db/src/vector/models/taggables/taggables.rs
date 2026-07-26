use serde::{Deserialize, Serialize};
use crate::vector::models::taggables::{subject::Subject, tag::Tag, topic::Topic};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Taggables {
    pub tags: Vec<Tag>,
    pub topic: Option<Topic>,
    pub subject: Option<Subject>,
}
