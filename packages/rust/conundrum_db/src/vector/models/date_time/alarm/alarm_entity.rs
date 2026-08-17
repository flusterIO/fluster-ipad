use std::sync::Arc;

use conundrum::{
    ecosystem::db::db_traits::{db_entity::DBSchema, db_field::DatabaseField},
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, specta::Type, Dummy)]
pub struct AlarmEntity {
    pub id: DatabaseId,
    pub alert_id: DatabaseId,
    pub time: DateTime,
}

impl<'a> DBSchema<'a> for AlarmEntity {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(DatabaseId::field_definition("alert_id", false)),
                Arc::new(DateTime::field_definition("time", false))])
    }
}
