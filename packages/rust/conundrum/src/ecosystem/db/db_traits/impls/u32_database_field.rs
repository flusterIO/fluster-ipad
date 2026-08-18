use arrow_schema::{DataType, Field};

use crate::ecosystem::db::db_traits::db_field::DatabaseField;

impl DatabaseField for u32 {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key.to_string(), DataType::UInt32, nullable)
    }
}
