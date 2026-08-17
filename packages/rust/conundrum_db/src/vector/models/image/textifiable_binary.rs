use conundrum::lifted_models::primitives::db_id::DatabaseId;

/// # TextifiableBinary
///
/// A wrapper around any Binary format that allows text to be extracted, like
/// audio files or video, whenever I get around to that level of text
/// extraction.
pub struct TextifiableBinary<T> {
    pub id: DatabaseId,
    pub data: T,
}
