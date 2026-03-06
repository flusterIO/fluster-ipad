use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, Clone, EnumIter, Serialize, Deserialize)]
pub enum BenchmarkMagnitude {
    Five = 5,
    Ten = 10,
    Twenty = 20,
    Fifty = 50,
}

#[typeshare]
#[derive(strum_macros::Display, uniffi::Enum, Clone, EnumIter, Serialize, Deserialize)]
/// Represents the date given some generated mdx benchmark content
/// as the content will need to grow as components are added.
pub enum BenchmarkGeneratedDateString {
    #[serde(rename = "2_2_2026")]
    #[strum(to_string = "2_2_2026")]
    Initial,
}

impl BenchmarkGeneratedDateString {
    pub fn get_latest() -> Self {
        BenchmarkGeneratedDateString::Initial
    }

    pub fn string_is_valid_date(s: &str) -> bool {
        BenchmarkGeneratedDateString::iter().any(|x| &x.to_string() == s)
    }
}
