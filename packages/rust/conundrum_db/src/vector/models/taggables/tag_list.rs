use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::taggables::tag::Tag;

/// ## TagList
///
/// This is a list of tags that can be applied to a number of resources in the
/// user's directory. Use these as the most generic way to link content within
/// the user's database.
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct TagList(Vec<Tag>);

impl TagList {
    pub fn from_strings(data: Vec<String>) -> Self {
        let input_data = data.iter().map(|s| Tag::from(s.clone())).collect::<Vec<Tag>>();
        Self(input_data)
    }
}
