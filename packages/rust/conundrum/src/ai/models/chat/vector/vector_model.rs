use std::sync::Arc;

use crate::ecosystem::db::db_traits::db_field::DatabaseField;
use arrow_schema::Field;
use fake::Dummy;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rig::embeddings::Embedding;

pub const DB_VECTOR_DIMENSIONS: i32 = 2560;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct DBVector(pub Vec<f64>);

impl Default for DBVector {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl From<Embedding> for DBVector {
    fn from(value: Embedding) -> Self {
        Self(value.vec)
    }
}

impl From<Vec<f64>> for DBVector {
    fn from(value: Vec<f64>) -> Self {
        Self(value)
    }
}

impl DatabaseField for DBVector {
    fn field_definition(field_key: &'static str, nullable: bool) -> Field {
        Field::new(field_key.to_string(),
                   arrow_schema::DataType::FixedSizeList(Arc::new(Field::new("item",
                                                                             arrow_schema::DataType::Float32,
                                                                             true)),
                                                         DB_VECTOR_DIMENSIONS),
                   nullable)
    }
}

impl DBVector {
    fn to_db_representation(&self) -> Vec<f64> {
        self.0.clone()
    }
}
