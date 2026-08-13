use arrow_schema::{DataType, Field};

use crate::vector::database::db_traits::db_field::DatabaseField;

impl DatabaseField for i64 {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key.to_string(), DataType::Int64, nullable)
    }
}

impl DatabaseField for i32 {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key.to_string(), DataType::Int32, nullable)
    }
}
