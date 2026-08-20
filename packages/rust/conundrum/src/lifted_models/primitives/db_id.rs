use std::fmt::Display;

use fake::{Dummy, Faker};
use lancedb::arrow::arrow_schema::Field;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{
    ecosystem::db::db_traits::{
        db_field::{DatabaseField, DatabaseFieldRepresentation},
        db_identifiable::DatabaseIdentifiable,
    },
    lifted_models::primitives::static_id::StaticId,
};

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
/// The developers of Surreal should be punched in the fucking eye. Make up your
/// mind. Is your db flexible or not? Make the types public or just be postgres
/// with pg-vector.
///
/// For anyone that sees this: Use Lance, or Neo, or something else. The only
/// reason I'm using surreal is because I was without internet for a few days
/// and I wanted to make progress, I already had Surreal installed. I'd go back
/// to Lance right now and undo 2 weeks worth of work just to get rid of this
/// half-axxed DB. I already miss the reliable arrow support of Lance instead of
/// serializing to sql strings and json objects. No wonder it's slow as shit.
/// It's the DB that does everything but nothing well.
pub struct DatabaseId(String);

impl DatabaseIdentifiable for DatabaseId {
    fn to_predicate(&self, field_key: &str) -> String {
        format!("{} = {}", field_key, self.0)
    }
}

impl Default for DatabaseId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for DatabaseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl DatabaseId {
    pub fn new() -> Self {
        DatabaseId(uuid::Uuid::new_v4().to_string())
    }

    pub fn new_from_input_id(value: String) -> Self {
        DatabaseId(value.clone())
    }
}

impl From<StaticId> for DatabaseId {
    fn from(value: StaticId) -> Self {
        Self(value.to_string())
    }
}

impl Dummy<Faker> for DatabaseId {
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        Self::new()
    }
}

impl DatabaseField for DatabaseId {
    fn field_definition(field_key: &'static str, nullable: bool) -> lancedb::arrow::arrow_schema::Field {
        Field::new(field_key, lancedb::arrow::arrow_schema::DataType::Utf8, nullable)
    }
}

impl DatabaseFieldRepresentation<String> for DatabaseId {
    fn to_db_representation(&self) -> String {
        self.0.clone()
    }
}
