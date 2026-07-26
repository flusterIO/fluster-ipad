use std::fmt::Display;

use conundrum::ecosystem::db::traits::database_field_representable::DatabaseFieldRepresentable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CaseInsensitiveString(String);

impl Display for CaseInsensitiveString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl DatabaseFieldRepresentable<String> for CaseInsensitiveString {
    fn to_db_representation(&self) -> String {
        self.0.clone()
    }
}
