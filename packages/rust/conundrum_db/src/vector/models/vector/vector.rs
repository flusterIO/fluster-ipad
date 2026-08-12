use std::sync::Arc;

use arrow_schema::Field;
use fake::Dummy;

pub const DB_VECTOR_DIMENSIONS: i32 = 768;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct DBVector(pub Vec<f64>);

impl Default for DBVector {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl DBVector {
    pub fn field_definition(nullable: bool) -> arrow_schema::Field {
        Field::new("vector",
                   arrow_schema::DataType::FixedSizeList(Arc::new(Field::new("item",
                                                                             arrow_schema::DataType::Float32,
                                                                             true)),
                                                         DB_VECTOR_DIMENSIONS),
                   nullable)
    }

    fn to_db_representation(&self) -> Vec<f64> {
        self.0.clone()
    }
}
