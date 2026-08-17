use arrow_schema::Field;
use conundrum::lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::date_time::alert::alert::Alert;

#[derive(Debug, Deserialize, Serialize, Clone, specta::Type, Dummy)]
pub struct Alarm {
    pub id: DatabaseId,
    pub alert: Alert,
    pub time: DateTime,
}
