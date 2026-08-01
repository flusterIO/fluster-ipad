use std::{fmt::Display, str::FromStr};

use conundrum::lang::runtime::state::conundrum_error::ConundrumError;
use fake::{Dummy, Faker};
use lancedb::arrow::arrow_schema::Field;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{
    test_utils::faker_generators::fake_words_as_string::fake_words_as_string,
    vector::database::db_traits::db_field::DatabaseField,
};

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct CaseInsensitiveString(String);

impl Dummy<String> for CaseInsensitiveString {
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(_: &String, _: &mut R) -> Self {
        let s = fake_words_as_string(0..10);
        CaseInsensitiveString(s)
    }
}

impl Dummy<Faker> for CaseInsensitiveString {
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(_: &Faker, _: &mut R) -> Self {
        let s = fake_words_as_string(0..10);
        CaseInsensitiveString(s)
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

impl Display for CaseInsensitiveString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CaseInsensitiveString {
    pub fn to_comparison_string(&self) -> String {
        self.0.to_lowercase()
    }
}

impl DatabaseField<(String, String), (Field, Field)> for CaseInsensitiveString {
    fn field_definition(field_key: &'static str,
                        nullable: bool)
                        -> (lancedb::arrow::arrow_schema::Field, lancedb::arrow::arrow_schema::Field) {
        (Field::new(field_key, lancedb::arrow::arrow_schema::DataType::Utf8, nullable),
         Field::new(format!("{}_lc", field_key), lancedb::arrow::arrow_schema::DataType::Utf8, nullable))
    }

    /// Returns the value, and the case-insensitive value in that order.
    fn to_db_representation(&self) -> (String, String) {
        (self.0.clone(), self.to_comparison_string())
    }
}
