use std::fmt::Display;

use conundrum::ecosystem::db::{
    tables::DatabaseTable, traits::database_field_representable::DatabaseFieldRepresentable,
};
use serde::{Deserialize, Serialize};
use surrealdb::types::{Kind, SurrealValue};

use crate::vector::database::db_traits::database_field::DatabaseField;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CaseInsensitiveString(String);

impl SurrealValue for CaseInsensitiveString {
    fn kind_of() -> surrealdb::types::Kind {
        Kind::String
    }

    fn into_value(self) -> surrealdb::types::Value {
        surrealdb::types::Value::String(self.0)
    }

    fn from_value(value: surrealdb::types::Value) -> Result<Self, surrealdb::Error>
        where Self: Sized {
        if let Some(res) = value.as_string() {
            Ok(Self(res.clone()))
        } else {
            Err(surrealdb::Error::thrown("Failed to deserialize string.".to_string()))
        }
    }
}

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
