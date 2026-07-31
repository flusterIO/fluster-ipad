use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use fake::{Dummy, Faker};
use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

use crate::vector::database::db_traits::database_field::{DatabaseField, OptionalDatabaseField};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AIGeneratedStatus {
    /// This was completely written by humans.
    None = 0,
    /// This is primarily human written, but with some AI generated content.
    Some = 1,
    /// Mostly AI written with some human input
    Most = 2,
    /// Completely AI written.
    All = 3,
}

impl TryFrom<i64> for AIGeneratedStatus {
    type Error = DatabaseError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Some),
            2 => Ok(Self::Most),
            3 => Ok(Self::All),
            _ => Err(DatabaseError::FailToSerialize("Invalid AIGeneratedStatus value.".to_string())),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<i64> for AIGeneratedStatus {
    fn into(self) -> i64 {
        match self {
            Self::None => 0,
            Self::Some => 1,
            Self::Most => 2,
            Self::All => 3,
        }
    }
}

impl SurrealValue for AIGeneratedStatus {
    fn kind_of() -> surrealdb_types::Kind {
        surrealdb_types::Kind::Int
    }

    fn into_value(self) -> surrealdb_types::Value {
        let n: i64 = self.into();
        surrealdb_types::Value::Number(surrealdb_types::Number::Int(n))
    }

    fn from_value(value: surrealdb_types::Value) -> Result<Self, surrealdb::Error>
        where Self: Sized {
        if let Some(n) = value.as_int() {
            let r = Self::try_from(*n).map_err(|e| {
                                          log::error!("Error: {:?}", e);
                                          surrealdb_types::Error::thrown(
                                                                         "Fail to serialize
AIGeneratedStatus."
                                                                         .to_string(),
                )
                                      })?;
            Ok(r)
        } else {
            Err(surrealdb_types::Error::thrown(
                                               "Fail to deserialize
AIGeneratedStatus."
                                               .to_string(),
            ))
        }
    }
}

impl Dummy<Faker> for AIGeneratedStatus {
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        if rng.random_ratio(1, 4) {
            AIGeneratedStatus::None
        } else if rng.random_ratio(1, 4) {
            AIGeneratedStatus::Some
        } else if rng.random_ratio(1, 4) {
            AIGeneratedStatus::Most
        } else {
            AIGeneratedStatus::All
        }
    }
}

impl DatabaseField for AIGeneratedStatus {
    fn field_definition(field_key: &'static str, table: &conundrum::ecosystem::db::tables::DatabaseTable) -> String {
        // TODO: Add assertion here to limit range.
        format!("DEFINE FIELD IF NOT EXISTS {} ON {} TYPE int", field_key, table)
    }
}

impl OptionalDatabaseField for AIGeneratedStatus {
    fn optional_field_definition(field_key: &'static str,
                                 table: &conundrum::ecosystem::db::tables::DatabaseTable)
                                 -> String {
        // TODO: Add assertion here to limit range.
        format!("DEFINE FIELD IF NOT EXISTS {} ON {} TYPE optional<int>", field_key, table)
    }
}
