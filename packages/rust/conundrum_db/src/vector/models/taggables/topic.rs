use fake::Dummy;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::vector::models::{
    date_time::date_time::DateTime, primitives::case_insensitive_string::CaseInsensitiveString,
    taggables::tag_location::TagLocation,
};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy, Type)]
pub struct Topic {
    pub value: CaseInsensitiveString,
    pub location: TagLocation,
    pub ctime: DateTime,
}

impl From<String> for Topic {
    fn from(value: String) -> Self {
        Topic { value: value.into(),
                location: TagLocation::Straggling,
                ctime: DateTime::new_now() }
    }
}
