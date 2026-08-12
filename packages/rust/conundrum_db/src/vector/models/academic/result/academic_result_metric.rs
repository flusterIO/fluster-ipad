use std::{collections::HashMap, ops::Index, str::FromStr};

use conundrum::ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult};
use fake::rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_traits::db_field::{DatabaseField, DatabaseFieldRepresentation},
    models::utility::{generic_value::GenericValue, generic_value_and_key::GenericValueAndKey},
};

#[derive(Serialize, Deserialize, Clone, Debug, strum_macros::Display, PartialEq, Eq)]
pub enum AcademicResultMetricKey {
    #[strum(to_string = "percent")]
    Percent,
    #[strum(to_string = "percent-error")]
    PercentError,
    #[strum(to_string = "rational-error")]
    RationalScore,
    #[strum(to_string = "standard-deviation")]
    StandardDeviation,
    #[strum(to_string = "custom")]
    Custom,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomAcademicResultMetric {
    pub score_type: String,
    pub value: f64,
}

impl CustomAcademicResultMetric {
    pub fn new(score_type: String, value: f64) -> CustomAcademicResultMetric {
        CustomAcademicResultMetric { score_type,
                                     value }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GeneralAcademicResultMetric {
    /// score_type must not include `AcademicResultMetricKey::Custum` or
    /// `AcademicResultMetricKey::RationalScore` as they have their own model.
    score_type: AcademicResultMetricKey,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RationalScore {
    pub numerator: f64,
    pub denominator: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AcademicResultMetric {
    /// A 100 based percentage, similar to score on a test.
    Percent(f64),
    /// An absolute value indicating the error percentage.
    PercentError(f64),
    /// A result on a test, like 64/70 when you don't want to convert it to a
    /// percentage.
    RationalScore(RationalScore),
    /// Standard Deviation
    StandardDeviation(f64),
    /// A custom metric, with the value and label.
    Custom(CustomAcademicResultMetric),
}

impl FromStr for AcademicResultMetric {
    type Err = DatabaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Figure out how to do this with try to deserialize the whole thing every
        // time.
        if let Ok(val) = serde_json::from_str::<GenericValueAndKey<f64, AcademicResultMetricKey>>(s) {
            match &val.key {
                AcademicResultMetricKey::StandardDeviation => return Ok(Self::StandardDeviation(val.value)),
                AcademicResultMetricKey::PercentError => return Ok(Self::PercentError(val.value)),
                AcademicResultMetricKey::Percent => return Ok(Self::Percent(val.value)),
                _ => {
                    log::error!("Malformed AcademicResultMetric encountered. Cannot continue.")
                }
            }
        } else if let Ok(val) = serde_json::from_str::<GenericValueAndKey<Vec<f64>, AcademicResultMetricKey>>(s) {
            if val.key == AcademicResultMetricKey::RationalScore {
                if val.value.len() == 2 {
                    return Ok(Self::RationalScore(RationalScore { numerator: *val.value.index(0),
                                                                  denominator: *val.value.index(1) }));
                } else {
                    log::error!("Malformed AcademicResultMetric encountered. Cannot continue.")
                }
            }
        } else if let Ok(val) = serde_json::from_str::<GenericValueAndKey<f64, String>>(s) {
            return Ok(Self::Custom(CustomAcademicResultMetric { score_type: val.key,
                                                                value: val.value }));
        }
        Err(DatabaseError::SerializationError)
    }
}

impl DatabaseFieldRepresentation<DatabaseResult<String>> for AcademicResultMetric {
    fn to_db_representation(&self) -> DatabaseResult<String> {
        match self {
            Self::Percent(n) => {
                let x = GenericValueAndKey { value: *n,
                                             key: AcademicResultMetricKey::Percent };
                let s = serde_json::to_string(&x).map_err(|e| {
                                                     log::error!("Error: {:?}", e);
                                                     DatabaseError::SerializationError
                                                 })?;
                Ok(s)
            }
            Self::PercentError(n) => {
                let x = GenericValueAndKey { value: *n,
                                             key: AcademicResultMetricKey::PercentError };
                let s = serde_json::to_string(&x).map_err(|e| {
                                                     log::error!("Error: {:?}", e);
                                                     DatabaseError::SerializationError
                                                 })?;
                Ok(s)
            }
            Self::RationalScore(n) => {
                let x = GenericValueAndKey { value: vec![n.numerator, n.denominator],
                                             key: AcademicResultMetricKey::RationalScore };
                let s = serde_json::to_string(&x).map_err(|e| {
                                                     log::error!("Error: {:?}", e);
                                                     DatabaseError::SerializationError
                                                 })?;
                Ok(s)
            }
            Self::StandardDeviation(n) => {
                let x = GenericValueAndKey { value: *n,
                                             key: AcademicResultMetricKey::StandardDeviation };
                let s = serde_json::to_string(&x).map_err(|e| {
                                                     log::error!("Error: {:?}", e);
                                                     DatabaseError::SerializationError
                                                 })?;
                Ok(s)
            }
            Self::Custom(l) => {
                let x = GenericValueAndKey { value: l.value,
                                             key: l.score_type.clone() };
                let s = serde_json::to_string(&x).map_err(|e| {
                                                     log::error!("Error: {:?}", e);
                                                     DatabaseError::SerializationError
                                                 })?;
                Ok(s)
            }
        }
    }
}

impl DatabaseField for AcademicResultMetricKey {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}

impl DatabaseField for AcademicResultMetric {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}
