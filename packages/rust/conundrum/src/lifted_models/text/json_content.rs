use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::ecosystem::{
    db::db_traits::db_field::{DatabaseField, DatabaseFieldLarge},
    error_handling::db_error::DatabaseError,
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct JsonContent(String);

impl TryInto<serde_json::Value> for JsonContent {
    type Error = DatabaseError;

    fn try_into(self) -> Result<serde_json::Value, Self::Error> {
        let val: serde_json::Value = serde_json::from_str(self.0.as_str()).map_err(|e| {
                                                                              log::error!("Serialization Error: {:?}",
                                                                                          e);
                                                                              DatabaseError::SerializationError
                                                                          })?;
        Ok(val)
    }
}

impl TryFrom<serde_json::Value> for JsonContent {
    type Error = DatabaseError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let s = serde_json::to_string(&value).map_err(|e| {
                                                 log::error!("Serialization Error: {:?}", e);
                                                 DatabaseError::SerializationError
                                             })?;
        Ok(Self(s))
    }
}

impl DatabaseField for JsonContent {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}

impl DatabaseFieldLarge for JsonContent {
    fn field_definition_large(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition_large(field_key, nullable)
    }
}
