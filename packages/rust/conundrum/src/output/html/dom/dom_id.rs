use std::fmt::Display;

use askama::FastWritable;
use serde::{Deserialize, Serialize};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DOMId(String);

uniffi::custom_newtype!(DOMId, String);

impl DOMId {
    pub fn new(id: String) -> Self {
        DOMId(id)
    }
}

impl Display for DOMId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FastWritable for DOMId {
    fn write_into<W: core::fmt::Write + ?Sized>(&self,
                                                dest: &mut W,
                                                values: &dyn askama::Values)
                                                -> askama::Result<()> {
        self.0.as_str().write_into(dest, values)
    }
}
