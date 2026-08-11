use arrow_schema::{DataType, FieldRef, UnionFields};
use conundrum::{
    ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult},
    parsers::conundrum::logic::number::{conundrum_float::ConundrumFloat, conundrum_int::ConundrumInt},
};
use fake::{
    Dummy, Fake, Faker,
    faker::{self, impls::lorem},
    rand::seq::IndexedRandom,
};
use lancedb::arrow::arrow_schema::Field;
use serde::{Deserialize, Serialize};
use serde_arrow::{
    schema::{SchemaLike, TracingOptions},
    utils::Item,
};
use std::{ops::Index, str::FromStr, sync::Arc};

use crate::vector::{
    database::db_traits::db_field::{DatabaseField, DatabaseFieldRepresentation},
    models::utility::generic_value::GenericValue,
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum FlashcardValue {
    Float(ConundrumFloat),
    Int(ConundrumInt),
    Text(String),
}

impl Dummy<Faker> for FlashcardValue {
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        // TODO: Improve this and actually return a genuinely random value.
        let s: String = Faker.fake();
        FlashcardValue::Text(s)
    }
}

impl From<i64> for FlashcardValue {
    fn from(value: i64) -> Self {
        let n: ConundrumInt = value.into();
        Self::Int(n)
    }
}

impl From<f64> for FlashcardValue {
    fn from(value: f64) -> Self {
        let n: ConundrumFloat = value.into();
        Self::Float(n)
    }
}

impl From<String> for FlashcardValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl FromStr for FlashcardValue {
    type Err = DatabaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(j) = serde_json::from_str::<GenericValue<String>>(s) {
            Ok(Self::Text(j.value))
        } else if let Ok(k) = serde_json::from_str::<GenericValue<f64>>(s) {
            let cf = ConundrumFloat::from(k.value);
            Ok(Self::Float(cf))
        } else if let Ok(l) = serde_json::from_str::<GenericValue<i64>>(s) {
            let ci = ConundrumInt::from(l.value);
            Ok(Self::Int(ci))
        } else {
            Ok(Self::Text(s.to_string()))
        }
    }
}

impl DatabaseFieldRepresentation<DatabaseResult<String>> for FlashcardValue {
    fn to_db_representation(&self) -> DatabaseResult<String> {
        match self {
            Self::Int(n) => {
                let x = GenericValue { value: n.0 };
                let json_string = serde_json::to_string(&x).map_err(|e| {
                                                               log::error!("Error: {:?}", e);
                                                               DatabaseError::SerializationError
                                                           })?;
                Ok(json_string)
            }
            Self::Float(n) => {
                let x = GenericValue { value: n.0 };
                let json_string = serde_json::to_string(&x).map_err(|e| {
                                                               log::error!("Error: {:?}", e);
                                                               DatabaseError::SerializationError
                                                           })?;
                Ok(json_string)
            }
            Self::Text(s) => {
                let x = GenericValue { value: s.to_string() };
                let json_string = serde_json::to_string(&x).map_err(|e| {
                                                               log::error!("Error: {:?}", e);
                                                               DatabaseError::SerializationError
                                                           })?;
                Ok(json_string)
            }
        }
    }
}

impl DatabaseField<Arc<Field>> for FlashcardValue {
    fn field_definition(field_key: &'static str, nullable: bool) -> Arc<lancedb::arrow::arrow_schema::Field> {
        let r = String::field_definition(field_key, nullable);
        Arc::new(r)
    }
}
