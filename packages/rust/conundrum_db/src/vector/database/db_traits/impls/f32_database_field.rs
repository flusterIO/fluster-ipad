use arrow_schema::{DataType, Field};

use crate::vector::database::db_traits::db_field::DatabaseField;

impl DatabaseField for f32 {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key.to_string(), DataType::Float32, nullable)
    }
}
