#[derive(Debug, serde::Serialize, Clone, Copy)]
pub struct ConundrumInt(pub i64);

uniffi::custom_newtype!(ConundrumInt, i64);

impl PartialEq<i64> for ConundrumInt {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}
