use crate::vector::database::db_traits::db_field::DatabaseField;
use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct AssignmentTag {
    pub tag_value: String,
    pub assignment_id: crate::vector::models::primitives::db_id::DatabaseId,
}
impl<'a> DBSchema<'a> for AssignmentTag {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(String::field_definition("tag_value", false)),
                Arc::new(<crate::vector::models::primitives::db_id::DatabaseId>::field_definition("assignment_id",
                                                                                                  false))])
    }
}
