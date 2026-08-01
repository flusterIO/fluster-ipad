use serde::{Deserialize, Serialize};

use crate::vector::models::academic::result::academic_result_metric::AcademicResultMetric;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NumericResult {
    pub label: String,
    pub desc: String,
    /// A JSON serialized string containing general meta data regarding this
    /// particular result. Remember... don't delete data that's there if you
    /// didn't put it there, because there might be multiple apps
    /// interacting with this DB.
    pub meta: String,
    pub accuracy: Option<AcademicResultMetric>,
}
