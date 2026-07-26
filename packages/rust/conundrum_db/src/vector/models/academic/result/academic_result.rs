use serde::{Deserialize, Serialize};

use crate::vector::models::{
    academic::result::academic_result_metric::AcademicResultMetric, taggables::taggables::Taggables,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct AcademicResult {
    pub result: AcademicResultMetric,
    pub taggables: Taggables,
}
