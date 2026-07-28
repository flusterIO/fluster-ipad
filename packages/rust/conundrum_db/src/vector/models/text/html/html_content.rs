use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;

use crate::vector::models::taggables::{subject::Subject, tag::Tag, topic::Topic};

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct HTMLContent {
    pub content: String,
    pub source_url: Option<String>,
    pub tags: Vec<Tag>,
    pub topic: Option<Topic>,
    pub subject: Option<Subject>,
}
