use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    date_time::date_time::DateTime,
    primitives::{case_insensitive_string::CaseInsensitiveString, db_id::DatabaseId},
    taggables::tag_location::TagLocation,
};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy)]
pub struct Subject {
    pub id: DatabaseId,
    pub value: CaseInsensitiveString,
    pub location: TagLocation,
    pub ctime: DateTime,
}

impl From<String> for Subject {
    fn from(value: String) -> Self {
        Subject { id: DatabaseId::new(),
                  value: CaseInsensitiveString::from(value),
                  location: TagLocation::Body,
                  ctime: DateTime::new_now() }
    }
}
