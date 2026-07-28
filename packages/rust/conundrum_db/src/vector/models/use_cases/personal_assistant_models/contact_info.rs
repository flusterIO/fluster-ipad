use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct PhysicalAddress {
    /// The street address.
    ///
    /// Example: 1 E Main Street
    pub street: Option<String>,
    /// The country as it appears in a typically written address.
    pub country: Option<String>,
    /// The string representation of the zip-code as it appears in a typically
    /// written address.
    pub zip: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct ContactInfo {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub address: Option<PhysicalAddress>,
    /// Optional conundrum content. Remember, this is for both AI and your own
    /// reference.
    pub note: Option<String>,
}
