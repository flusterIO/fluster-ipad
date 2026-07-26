use conundrum::ecosystem::db::tables::DatabaseTable;
use serde::{Deserialize, Serialize};

use crate::vector::database::{
    db_traits::enum_variant_db_models::EnumDBModelVariantMethods, db_types::db_entity::DBEntity,
};

#[derive(Serialize, strum_macros::Display)]
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

#[derive(Serialize, Clone, Debug, Deserialize)]
pub enum AcademicResultMetric {
    /// A 100 based percentage, similar to score on a test.
    Percent(f32),
    /// An absolute value indicating the error percentage.
    PercentError(u32),
    /// A result on a test, like 64/70 when you don't want to convert it to a
    /// percentage.
    RationalScore {
        numerator: u32,
        denominator: u32,
    },
    /// Standard Deviation
    StandardDeviation(f32),
    /// A custom metric, with the value and label.
    Custom {
        value: f32,
        label: String,
    },
}

impl EnumDBModelVariantMethods for AcademicResultMetric {
    fn schema(&self) -> String {
        todo!()
    }

    fn table(&self) -> conundrum::ecosystem::db::tables::DatabaseTable {
        match self {
            Self::Percent(_) => DatabaseTable::NumericAcademicResultMetric,
            Self::PercentError(_) => DatabaseTable::NumericAcademicResultMetric,
            Self::RationalScore { numerator,
                                  denominator, } => DatabaseTable::RationalScoreAcademicResultMetric,
            Self::StandardDeviation(_) => DatabaseTable::NumericAcademicResultMetric,
            Self::Custom { value,
                           label, } => DatabaseTable::CustomAcademicResultMetric,
        }
    }
}

pub struct ComposedAcademicResultMetric {
    pub key: AcademicResultMetricKey,
    pub metric: AcademicResultMetric,
}
