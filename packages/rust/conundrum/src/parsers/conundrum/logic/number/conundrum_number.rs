use serde::Serialize;

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum ConundrumNumber {
    Int(i128),
    Float(f64),
}

impl ConundrumNumber {
    /// Return the number as an f64, unchanged if already
    /// `ConundrumNumber::Float`, else just a simple type cast.
    pub fn as_float(&self) -> f64 {
        match self {
            ConundrumNumber::Int(n) => *n as f64,
            ConundrumNumber::Float(n) => *n,
        }
    }
}
