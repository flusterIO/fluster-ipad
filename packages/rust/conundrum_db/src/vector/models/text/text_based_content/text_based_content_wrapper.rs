use std::fmt::Debug;

use fake::{Dummy, Faker};
use serde::{Deserialize, Serialize};

use crate::vector::models::text::text_based_content::text_based_content_trait::TextBasedContent;

pub trait TextBasedContentWrapper<'a, ParseParameters, ChunkType> {
    type ContentType: TextBasedContent<ParseParameters, ChunkType>
        + Serialize
        + Deserialize<'a>
        + Dummy<Faker>
        + Debug
        + Clone
        + specta::Type;
    fn inner_text(&self) -> String;
}
