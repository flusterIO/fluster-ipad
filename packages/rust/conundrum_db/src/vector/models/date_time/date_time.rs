use crate::vector::database::db_traits::database_field::DatabaseField;
use chrono::Utc;
use fake::{Dummy, Fake, Faker, faker::chrono::en::DateTime as FakeChronoDateTime};
use serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, Error, Kind, SurrealValue, Value};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DateTime(chrono::DateTime<Utc>);

impl Dummy<Faker> for DateTime {
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let dt: chrono::DateTime<Utc> = FakeChronoDateTime().fake();
        Self(dt)
    }
}

impl DatabaseField for DateTime {
    fn field_definition(field_key: &'static str, table: &conundrum::ecosystem::db::tables::DatabaseTable) -> String {
        format!("DEFINE FIELD IF NOT EXISTS {} ON {} TYPE datetime;", field_key, table)
    }
}

impl DateTime {
    pub fn new_now() -> DateTime {
        let d = Utc::now();
        DateTime(d)
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
