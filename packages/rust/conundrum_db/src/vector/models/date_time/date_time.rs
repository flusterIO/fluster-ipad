use crate::vector::database::db_traits::database_field::DatabaseField;
use chrono::Utc;
use conundrum::ecosystem::db::traits::database_field_representable::DatabaseFieldRepresentable;
use serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, Error, Kind, SurrealValue, ToSql, Value};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DateTime(chrono::DateTime<Utc>);

impl DatabaseField for DateTime {
    fn field_definition(field_key: &'static str, table: &conundrum::ecosystem::db::tables::DatabaseTable) -> String {
        format!("DEFINE FIELD {} ON {} TYPE datetime;", field_key, table)
    }
}

impl DateTime {
    pub fn new_now() -> DateTime {
        let d = Utc::now();
        DateTime(d)
    }
}

impl DatabaseFieldRepresentable<i64> for DateTime {
    fn to_db_representation(&self) -> i64 {
        self.0.timestamp_millis()
    }
}

impl SurrealValue for DateTime {
    fn kind_of() -> Kind {
        Kind::Datetime
    }

    fn into_value(self) -> Value {
        Value::Datetime(Datetime::from(self.0))
    }

    fn from_value(value: Value) -> Result<Self, Error>
        where Self: Sized {
        match value {
            Value::Datetime(n) => {
                let input = n.to_utc();
                Ok(DateTime(input))
            }
            _ => Err(Error::thrown("No good".to_string())),
        }
    }
}
