use serde::{Deserialize, Serialize};

use crate::lang::elements::parsed_code_block::ParsedCodeBlock;

#[derive(uniffi::Enum, Serialize, Deserialize, Debug, Clone)]
pub enum AiSerializationRequestType {
    CreateNoteSpecificStudyGuide,
    SummarizeNote,
    RecommendResearch,
}

#[derive(uniffi::Record, Debug, Serialize, Deserialize, Clone)]
pub struct AiSerializationRequestPhase1 {
    pub parsing_result: ParsedCodeBlock,
}
