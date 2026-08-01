use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EcosystemSettingValue {
    GenericString(String),
    Float(f64),
    Int(i64),
}
