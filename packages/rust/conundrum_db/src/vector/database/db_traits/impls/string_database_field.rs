use arrow_schema::Field;

use crate::vector::database::db_traits::db_field::{DatabaseField, DatabaseFieldLarge};

impl DatabaseField for String {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key, arrow_schema::DataType::Utf8, nullable)
    }
}

impl DatabaseFieldLarge for String {
    fn field_definition_large(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key, arrow_schema::DataType::LargeUtf8, nullable)
    }
}
