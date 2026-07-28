use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_traits::{composed_model::ComposedModel, pure_model_instance::PureModelInstanceMethods},
    models::taggables::tag::Tag,
};

/// ## TagList
///
/// You guessed it... just a list of tags and some utility methods.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TagList(Vec<Tag>);
