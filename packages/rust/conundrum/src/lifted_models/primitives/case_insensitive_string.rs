use std::{fmt::Display, str::FromStr};

use fake::{Dummy, Faker};
use lancedb::arrow::arrow_schema::Field;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{
    ecosystem::db::db_traits::{db_field::DatabaseField, db_identifiable::DatabaseIdentifiable},
    lang::{lib::std_lib_impls::json_string::QuotedString, runtime::state::conundrum_error::ConundrumError},
    testing::faker_generators::fake_words_as_string::fake_words_as_string,
};

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct CaseInsensitiveString(String);

impl DatabaseIdentifiable for CaseInsensitiveString {
    fn to_predicate(&self, field_key: &str) -> String {
        // TODO: I'm pretty sure there's some syntax to cast the data to lowercase as
        // well. This will be broken without that.
        format!("{} = {}", field_key, self.0.to_lowercase().to_quoted_string_with_fallback())
    }
}

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
