use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ConundrumNumber {
    Int(i64),
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
