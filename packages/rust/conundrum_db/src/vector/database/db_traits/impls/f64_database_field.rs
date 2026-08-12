use arrow_schema::{DataType, Field};

use crate::vector::database::db_traits::db_field::DatabaseField;

impl DatabaseField for f64 {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key.to_string(), DataType::Float64, nullable)
    }
}


impl DatabaseField for Vec<f64> {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key.to_string(), DataType::List(std::sync::Arc::new(
                    Field::new("item", DataType::Float64, true)
        )), nullable)
    }
}

