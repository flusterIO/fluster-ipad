use crate::vector::models::text::text_based_content::text_based_content_trait::TextBasedContent;

pub trait TextBasedContentWrapper<ParseParameters, ChunkType> {
    type ContentType: TextBasedContent<ParseParameters, ChunkType>;
    fn inner_text(&self) -> String;
}
