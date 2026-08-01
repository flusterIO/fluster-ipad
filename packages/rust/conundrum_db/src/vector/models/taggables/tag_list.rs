use serde::{Deserialize, Serialize};

use crate::vector::models::taggables::tag::Tag;

/// ## TagList
///
/// You guessed it... just a list of tags and some utility methods.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TagList(Vec<Tag>);

impl TagList {
    pub fn from_strings(data: Vec<String>) -> Self {
        let input_data = data.iter().map(|s| Tag::from(s.clone())).collect::<Vec<Tag>>();
        Self(input_data)
    }
}
