use serde::{Deserialize, Serialize};

use crate::vector::models::{
    date_time::alert::alert::Alert,
    lifestyle::life_connections::models::person::Person,
    taggables::{subject::Subject, tag::Tag, topic::Topic},
};

/// ## ClassModel
///
/// A school class, not a Rust class. Oh do I wish we had a Rust class.
#[derive(Serialize, Deserialize, Clone)]
pub struct ClassModel {
    /// Can be conundrum content, but should be parsed with the
    /// `perferInlineSyntax` flag.
    pub label: String,
    pub teachers: Vec<Person>,
    // If this is going to be local first, it doesn't make sense to structure this so that the
    // classmates are derived from a shared class, since it won't be shared.
    pub classmates: Vec<Person>,
    pub tags: Vec<Tag>,
    pub topic: Option<Topic>,
    pub subject: Option<Subject>,
    pub alerts: Vec<Alert>,
}
