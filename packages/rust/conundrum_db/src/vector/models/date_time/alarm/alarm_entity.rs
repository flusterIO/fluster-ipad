use std::sync::Arc;

use conundrum::{
    ecosystem::db::{
        db_traits::{db_entity::DBSchema, db_field::DatabaseField},
        tables::DatabaseTable,
    },
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::Alarm)]
pub struct AlarmEntity {
    pub id: DatabaseId,
    pub alert_id: DatabaseId,
    pub time: DateTime,
}
