use arrow_schema::Field;

use crate::ecosystem::db::db_traits::db_field::{DatabaseField, DatabaseFieldLarge};

/// # Bytes
///
/// Any binary data and some utility methods.
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, specta::Type, fake::Dummy)]
pub struct Bytes(pub Vec<u8>);

impl DatabaseField for Bytes {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key.to_string(), arrow_schema::DataType::Binary, nullable)
    }
}

impl DatabaseFieldLarge for Bytes {
    fn field_definition_large(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key.to_string(), arrow_schema::DataType::LargeBinary, nullable)
    }
}
