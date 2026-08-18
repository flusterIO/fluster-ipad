use std::{fmt::Display, str::FromStr};

use crate::ecosystem::{
    db::db_traits::db_field::{DatabaseField, DatabaseFieldRepresentation},
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use chrono::Utc;
use fake::{Dummy, Fake, Faker, faker::chrono::en::DateTime as FakeChronoDateTime};
use lancedb::arrow::arrow_schema::Field;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use specta::Type;

/// # DateTime
///
/// A simple wrapper around a UTC timestamp in milliseconds.
#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(transparent)]
pub struct DateTime(#[specta(type = String)] i64);

impl Dummy<Faker> for DateTime {
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let dt: chrono::DateTime<Utc> = FakeChronoDateTime().fake();
        let s = dt.timestamp_millis();
        Self(s)
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for DateTime {
    type Err = DatabaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: i64 = s.parse().map_err(|e| {
                                   log::error!("DateTime Serialization Error: {:?}", e);
                                   DatabaseError::SerializationError
                               })?;
        Ok(Self(n))
    }
}

impl From<i64> for DateTime {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl DateTime {
    pub fn new_now() -> DateTime {
        let d = Utc::now();
        DateTime(d.timestamp_millis())
    }

    pub fn to_chrono(&self) -> DatabaseResult<chrono::DateTime<Utc>> {
        chrono::DateTime::from_timestamp_millis(self.0).ok_or(DatabaseError::InvalidDateTime)
    }
}

impl DatabaseField for DateTime {
    fn field_definition(field_key: &'static str, nullable: bool) -> lancedb::arrow::arrow_schema::Field {
        Field::new(field_key, lancedb::arrow::arrow_schema::DataType::Timestamp(lancedb::arrow::arrow_schema::TimeUnit::Millisecond, Some("UTC".to_string().into()) ), nullable)
    }
}

impl DatabaseFieldRepresentation<i64> for DateTime {
    fn to_db_representation(&self) -> i64 {
        self.0
    }
}
