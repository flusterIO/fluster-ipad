use serde::{Deserialize, Serialize};

use crate::vector::models::{
    academic::result::academic_result_metric::AcademicResultMetric,
    primitives::db_id::DatabaseId,
    taggables::{subject::Subject, tag::Tag, topic::Topic},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct AcademicResult {
    pub id: DatabaseId,
    pub result: AcademicResultMetric,
    pub tags: Vec<Tag>,
    pub topic: Option<Topic>,
    pub subject: Option<Subject>,
}
