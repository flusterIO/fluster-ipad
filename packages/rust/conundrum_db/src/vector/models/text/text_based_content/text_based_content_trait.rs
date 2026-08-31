use conundrum::{
    ecosystem::{
        db::db_traits::db_field::DatabaseField,
        error_handling::{ai_error::AIResult, db_error::DatabaseResult},
    },
    lang::runtime::state::parse_state::{ConundrumCompileTarget, ConundrumModifier},
};

use crate::vector::models::text::text_based_content::text_based_chunk::TextBasedChunk;

pub trait TextBasedContent<ParseParameters, ChunkType = TextBasedChunk>: DatabaseField {
    /// Returns a markdown string.
    fn inner_text(&self) -> String;
    async fn get_parsed_content(&self, opts: ParseParameters) -> AIResult<String>;
    async fn get_title(&self,
                       modifiers: Vec<ConundrumModifier>,
                       target: ConundrumCompileTarget)
                       -> AIResult<Option<String>>;
}
