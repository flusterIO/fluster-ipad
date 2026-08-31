use crate::vector::models::binary::binary_based_content_trait::BinaryBasedContent;

pub trait BinaryBasedContentWrapper<ParseParameters, ChunkType> {
    type ContentType: BinaryBasedContent<ParseParameters, ChunkType>;
    fn to_bytes(&self) -> Vec<u8>;
}
