use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

use crate::vector::models::{
    date_time::alert::alert::Alert,
    taggables::{subject::Subject, tag::Tag, topic::Topic},
};

/// ## ClassModel
///
/// A school class, not a Rust class. Oh do I wish we had a Rust class.
#[derive(Serialize, Deserialize, Clone, SurrealValue)]
pub struct ClassModel {
    /// Can be conundrum content, but should be parsed with the
    /// `perferInlineSyntax` flag.
    pub label: String,
    pub tags: Vec<Tag>,
    pub topic: Option<Topic>,
    pub subject: Option<Subject>,
    pub alerts: Vec<Alert>,
}
