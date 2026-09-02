use arrow_schema::Field;
use conundrum::{
    ecosystem::db::tables::DatabaseTable,
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::date_time::alert::alert::Alert;

#[derive(Debug, Deserialize, Serialize, Clone, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::Alarm)]
pub struct Alarm {
    pub id: DatabaseId,
    pub alert_id: DatabaseId,
    pub time: DateTime,
}
