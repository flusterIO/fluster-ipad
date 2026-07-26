use serde::{Deserialize, Serialize};

use crate::vector::models::taggables::taggables::Taggables;

#[derive(Serialize, Deserialize, Clone)]
pub struct AcademicResult {
    pub taggables: Taggables,
}
