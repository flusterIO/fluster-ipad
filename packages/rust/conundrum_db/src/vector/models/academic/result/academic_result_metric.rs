use serde::{Deserialize, Serialize};
use surrealdb::types::{SurrealValue, kind};
use surrealdb_types::{Error, Number, Object, Value};

#[derive(Serialize, Deserialize, Clone, Debug, strum_macros::Display, SurrealValue)]
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

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
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

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct GeneralAcademicResultMetric {
    /// score_type must not include `AcademicResultMetricKey::Custum` or
    /// `AcademicResultMetricKey::RationalScore` as they have their own model.
    score_type: AcademicResultMetricKey,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct RationalScore {
    score_type: AcademicResultMetricKey,
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

impl SurrealValue for AcademicResultMetric {
    fn kind_of() -> surrealdb::types::Kind {
        kind!({ score_type: string, numerator: number, denominator: number } | { score_type: string, value: number } | { score_type: string, label: string, value: number })
    }

    fn into_value(self) -> surrealdb::types::Value {
        surrealdb_types::Value::Object(self.to_object())
    }

    fn from_value(value: surrealdb::types::Value) -> Result<Self, surrealdb::Error>
        where Self: Sized {
        let Value::Object(obj) = value else {
            return Err(Error::thrown("Something was not an object".to_string()));
        };
        if let Some(score_type) = obj.get("score_type") {
            if let Some(s) = score_type.as_string() {
                if *s == AcademicResultMetricKey::RationalScore.to_string() {
                    let numerator =
                        obj.get("numerator")
                           .ok_or_else(|| Error::thrown("Invalid numerator in the AcademicResultMetric".to_string()))?
                           .as_number()
                           .ok_or_else(|| Error::thrown("Invalid numerator in the AcademicResultMetric".to_string()))?
                           .to_f64()
                           .ok_or_else(|| Error::thrown("Invalid numerator in the AcademicResultMetric".to_string()))?;
                    let denominator =
                        obj.get("denominator")
                           .ok_or_else(|| Error::thrown("Invalid denominator in the AcademicResultMetric".to_string()))?
                           .as_number()
                           .ok_or_else(|| Error::thrown("Invalid denominator in the AcademicResultMetric".to_string()))?
                           .to_f64()
                           .ok_or_else(|| {
                               Error::thrown("Invalid denominator in the AcademicResultMetric".to_string())
                           })?;
                    Ok(Self::RationalScore(RationalScore { score_type: AcademicResultMetricKey::RationalScore,
                                                           numerator,
                                                           denominator }))
                } else if *s == AcademicResultMetricKey::Percent.to_string() {
                    let val =
                        obj.get("value")
                           .ok_or_else(|| Error::thrown("Invalid value in the AcademicResultMetric".to_string()))?
                           .as_f64()
                           .ok_or_else(|| Error::thrown("Invalid value in the AcademicResultMetric".to_string()))?;
                    Ok(Self::Percent(*val))
                } else if *s == AcademicResultMetricKey::StandardDeviation.to_string() {
                    let val =
                        obj.get("value")
                           .ok_or_else(|| Error::thrown("Invalid value in the AcademicResultMetric".to_string()))?
                           .as_f64()
                           .ok_or_else(|| Error::thrown("Invalid value in the AcademicResultMetric".to_string()))?;
                    Ok(Self::StandardDeviation(*val))
                } else if *s == AcademicResultMetricKey::PercentError.to_string() {
                    let val =
                        obj.get("value")
                           .ok_or_else(|| Error::thrown("Invalid value in the AcademicResultMetric".to_string()))?
                           .as_f64()
                           .ok_or_else(|| Error::thrown("Invalid value in the AcademicResultMetric".to_string()))?;
                    Ok(Self::PercentError(*val))
                } else {
                    let score_type =
                        obj.get("score_type")
                           .ok_or_else(|| {
                               Error::thrown("Invalid score_type type in the AcademicResultMetric.".to_string())
                           })?
                           .as_string()
                           .ok_or_else(|| {
                               Error::thrown("Invalid score_type type in the AcademicResultMetric.".to_string())
                           })?;
                    let val =
                        obj.get("value")
                           .ok_or_else(|| Error::thrown("Invalid value in the AcademicResultMetric".to_string()))?
                           .as_f64()
                           .ok_or_else(|| Error::thrown("Invalid value in the AcademicResultMetric".to_string()))?;
                    Ok(Self::Custom(CustomAcademicResultMetric { score_type: score_type.clone(),
                                                                 value: *val }))
                }
            } else {
                Err(Error::thrown("Invalid score_type field.".to_string()))
            }
        } else {
            Err(Error::thrown("Invalid score_type field.".to_string()))
        }
    }
}

impl AcademicResultMetric {
    pub fn to_object(&self) -> Object {
        let mut obj = Object::new();
        match self {
            Self::Percent(p) => {
                obj.insert("score_type", AcademicResultMetricKey::Percent);
                obj.insert("value", Number::from_float(*p));
            }
            Self::StandardDeviation(s) => {
                obj.insert("score_type", AcademicResultMetricKey::StandardDeviation);
                obj.insert("value", Number::from_float(*s));
            }
            Self::PercentError(p) => {
                obj.insert("score_type", AcademicResultMetricKey::PercentError);
                obj.insert("value", Number::from_float(*p));
            }
            Self::RationalScore(s) => {
                obj.insert("score_type", AcademicResultMetricKey::RationalScore);
                obj.insert("numerator", Number::from_float(s.numerator));
                obj.insert("denominator", Number::from_float(s.denominator));
            }
            Self::Custom(c) => {
                obj.insert("score_type", c.score_type.to_string());
                obj.insert("value", Number::from_float(c.value));
            }
        }
        obj
    }
}
