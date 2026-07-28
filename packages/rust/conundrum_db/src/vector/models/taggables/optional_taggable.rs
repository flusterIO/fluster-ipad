use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_traits::composed_model::ComposedModelOptionalField,
    models::taggables::{subject::Subject, topic::Topic},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OptionalTaggable<T>(Option<T>);

impl ComposedModelOptionalField<Topic> for OptionalTaggable<Topic> {
    fn to_optional_pure_model(&self) -> Option<Topic> {
        self.0.clone()
    }
}

impl ComposedModelOptionalField<Subject> for OptionalTaggable<Subject> {
    fn to_optional_pure_model(&self) -> Option<Subject> {
        self.0.clone()
    }
}
