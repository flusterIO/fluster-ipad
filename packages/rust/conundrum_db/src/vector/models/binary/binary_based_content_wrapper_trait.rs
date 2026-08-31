use std::fmt::Debug;

use fake::{Dummy, Faker};
use serde::{Deserialize, Serialize};

use crate::vector::models::binary::binary_based_content_trait::BinaryBasedContent;

pub trait BinaryBasedContentWrapper<'a, ParseParameters, ChunkType> {
    type ContentType: BinaryBasedContent<ParseParameters, ChunkType>
        + Serialize
        + Deserialize<'a>
        + Dummy<Faker>
        + Debug
        + Clone
        + specta::Type;
    fn to_bytes(&self) -> Vec<u8>;
}
