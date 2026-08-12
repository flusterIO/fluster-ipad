use crate::{topic_join, vector::models::primitives::db_id::DatabaseId};

use crate::vector::database::db_traits::db_field::DatabaseField;
use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct AssignmentTopic {
    pub topic_value: String,
    pub topic_id: DatabaseId,
}
impl<'a> DBSchema<'a> for AssignmentTopic {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(String::field_definition("topic_value", false)),
                Arc::new(<DatabaseId>::field_definition("topic_id", false))])
    }
}
