use arrow_schema::Field;

use crate::ecosystem::db::db_traits::db_field::DatabaseField;

impl DatabaseField for bool {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key.to_string(), arrow_schema::DataType::Boolean, nullable)
    }
}
