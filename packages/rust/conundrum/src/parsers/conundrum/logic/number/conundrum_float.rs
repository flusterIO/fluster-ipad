use uniffi::TypeId;
#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone, Copy)]
pub struct ConundrumFloat(pub f64);

uniffi::custom_newtype!(ConundrumFloat, f64);
