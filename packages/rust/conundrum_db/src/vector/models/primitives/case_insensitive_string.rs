use std::{fmt::Display, str::FromStr};

use conundrum::{
    ecosystem::db::{tables::DatabaseTable, traits::database_field_representable::DatabaseFieldRepresentable},
    lang::runtime::state::conundrum_error::ConundrumError,
};
use fake::{Dummy, Faker};
use serde::{Deserialize, Serialize};
use surrealdb::types::{Kind, SurrealValue};

use crate::{
    test_utils::faker_generators::fake_words_as_string::fake_words_as_string,
    vector::database::db_traits::database_field::DatabaseField,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CaseInsensitiveString(String);

impl Dummy<String> for CaseInsensitiveString {
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &String, rng: &mut R) -> Self {
        let s = fake_words_as_string(0..10);
        CaseInsensitiveString(s)
    }
}

impl Dummy<Faker> for CaseInsensitiveString {
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let s = fake_words_as_string(0..10);
        CaseInsensitiveString(s)
    }
}

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

impl FromStr for CaseInsensitiveString {
    type Err = ConundrumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CaseInsensitiveString(s.to_string()))
    }
}

impl From<String> for CaseInsensitiveString {
    fn from(value: String) -> Self {
        CaseInsensitiveString(value)
    }
}

impl DatabaseField for CaseInsensitiveString {
    fn field_definition(field_key: &'static str, table: &DatabaseTable) -> String {
        format!("DEFINE FIELD IF NOT EXISTS {} ON {} TYPE string;", field_key, table)
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

impl CaseInsensitiveString {
    pub fn to_comparison_string(&self) -> String {
        self.0.to_lowercase()
    }
}
