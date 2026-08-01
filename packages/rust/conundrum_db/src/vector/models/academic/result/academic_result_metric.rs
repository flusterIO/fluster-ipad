use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, strum_macros::Display)]
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
