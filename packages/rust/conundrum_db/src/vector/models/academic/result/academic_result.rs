use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;

use crate::vector::models::{
    academic::result::academic_result_metric::AcademicResultMetric,
    taggables::{subject::Subject, tag::Tag, topic::Topic},
};

#[derive(Serialize, Deserialize, Clone, SurrealValue)]
pub struct AcademicResult {
    pub result: AcademicResultMetric,
    pub tags: Vec<Tag>,
    pub topic: Option<Topic>,
    pub subject: Option<Subject>,
}
