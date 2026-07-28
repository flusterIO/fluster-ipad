use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

use crate::vector::models::{
    date_time::alert::alert::Alert,
    taggables::{subject::Subject, tag::Tag, topic::Topic},
    use_cases::personal_assistant_models::contact_info::ContactInfo,
};

/// ## ClassModel
///
/// A school class, not a Rust class. Oh do I wish we had a Rust class.
#[derive(Serialize, Deserialize, Clone, SurrealValue)]
pub struct ClassModel {
    /// Can be conundrum content, but should be parsed with the
    /// `perferInlineSyntax` flag.
    pub label: String,
    pub teachers: Vec<ContactInfo>,
    // If this is going to be local first, it doesn't make sense to structure this so that the
    // classmates are derived from a shared class, since it won't be shared.
    pub classmates: Vec<ContactInfo>,
    pub tags: Vec<Tag>,
    pub topic: Option<Topic>,
    pub subject: Option<Subject>,
    pub alerts: Vec<Alert>,
}
