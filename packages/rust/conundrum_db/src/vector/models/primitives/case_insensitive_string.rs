use std::fmt::Display;

use conundrum::ecosystem::db::{
    tables::DatabaseTable, traits::database_field_representable::DatabaseFieldRepresentable,
};
use serde::{Deserialize, Serialize};

use crate::vector::database::db_traits::database_field::DatabaseField;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CaseInsensitiveString(String);

impl DatabaseField for CaseInsensitiveString {
    fn field_definition(field_key: &'static str, table: &DatabaseTable) -> String {
        format!("DEFINE FIELD {} ON {} TYPE string;", field_key, table)
    }
}

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
