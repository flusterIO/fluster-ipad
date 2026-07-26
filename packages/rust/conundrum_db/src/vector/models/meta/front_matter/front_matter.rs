use serde::{Deserialize, Serialize};

use crate::vector::models::taggables::{subject::Subject, tag::Tag, topic::Topic};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FrontMatter {
    pub summary: Option<String>,
    pub tags: Option<Tag>,
    pub topic: Option<Topic>,
    pub subject: Option<Subject>,
}
