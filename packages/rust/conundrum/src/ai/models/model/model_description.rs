use rig::model::Model;
use serde::{Deserialize, Serialize};

use crate::lifted_models::primitives::date_time::DateTime;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
pub struct ModelDescription {
    pub id: String,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub model_type: Option<String>,
    pub ctime: Option<DateTime>,
    pub max_context: Option<u32>,
}

impl From<Model> for ModelDescription {
    fn from(value: Model) -> Self {
        ModelDescription { id: value.id.clone(),
                           name: value.name.clone(),
                           desc: value.description.clone(),
                           model_type: value.r#type.clone(),
                           ctime: value.created_at.map(|x| DateTime::from(x)),
                           max_context: value.context_length }
    }
}
