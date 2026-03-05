use fluster_core_utilities::core_types::ai::CodeBlockParsingResult;
use serde::{Deserialize, Serialize};

#[derive(uniffi::Enum, Serialize, Deserialize, Debug, Clone)]
pub enum AiSerializationRequestType {
    CreateNoteSpecificStudyGuide,
    SummarizeNote,
    RecommendResearch,
}

#[derive(uniffi::Record, Debug, Serialize, Deserialize, Clone)]
pub struct AiSerializationRequestPhase1 {
    pub parsing_result: CodeBlockParsingResult,
}
