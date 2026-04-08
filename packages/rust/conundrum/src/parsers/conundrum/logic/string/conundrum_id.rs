/// A utility struct with some additional methods for working with id's in
/// Conundrum.
pub struct ConundrumId(pub String);

impl ConundrumId {
    pub fn validate(&self) -> bool {
        // TODO: FIx this... actually validate the id on the parser level and don't just
        // wait for TS to blow up.
        true
    }
}
