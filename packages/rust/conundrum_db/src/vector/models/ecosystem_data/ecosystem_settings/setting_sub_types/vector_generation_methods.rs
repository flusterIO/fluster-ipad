use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::ecosystem_data::ecosystem_settings::setting_sub_types::vector_generation_method::VectorGenerationMethod;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct VectorGenerationMethods {
    /// The vector generation settings for system logs.
    pub logs: Option<VectorGenerationMethod>,
}
