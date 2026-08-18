use std::{fmt::Display, str::FromStr};

use fake::{Dummy, Faker};
use lancedb::arrow::arrow_schema::Field;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{
    ecosystem::db::db_traits::db_field::DatabaseField, lang::runtime::state::conundrum_error::ConundrumError,
    testing::faker_generators::fake_words_as_string::fake_words_as_string,
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

impl DatabaseField<Field> for CaseInsensitiveString {
    fn field_definition(field_key: &'static str, nullable: bool) -> lancedb::arrow::arrow_schema::Field {
        Field::new(field_key, lancedb::arrow::arrow_schema::DataType::Utf8, nullable)
    }
}
