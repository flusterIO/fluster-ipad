use arrow_schema::{Field, Fields};
use serde::{Deserialize, Serialize};

use crate::vector::database::db_traits::db_field::DatabaseField;

// Because Lance refuses to serialize to all of arrow. #[derive(Serialize,
// Deserialize, Clone, Debug)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GenericValueAndKey<T: DatabaseField, K: DatabaseField> {
    pub value: T,
    pub key: K,
}

impl<T: DatabaseField, K: DatabaseField> DatabaseField for GenericValueAndKey<T, K> {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key.to_string(),
                   arrow_schema::DataType::Struct(Fields::from(vec![T::field_definition("value", false),
                                                                    K::field_definition("key", false),])),
                   nullable)
    }
}
