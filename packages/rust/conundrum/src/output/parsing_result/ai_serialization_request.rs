use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::parsers::markdown::code_block::code_block::ParsedCodeBlock;

#[typeshare]
#[derive(uniffi::Enum, Serialize, Deserialize, Debug, Clone)]
pub enum AiSerializationRequestType {
    CreateNoteSpecificStudyGuide,
    SummarizeNote,
    RecommendResearch,
}

#[typeshare]
#[derive(uniffi::Record, Debug, Serialize, Deserialize, Clone)]
pub struct AiSerializationRequestPhase1 {
    pub parsing_result: ParsedCodeBlock,
}
