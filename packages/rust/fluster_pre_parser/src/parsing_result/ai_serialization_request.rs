use serde::{Deserialize, Serialize};

#[derive(uniffi::Enum, Serialize, Deserialize, Debug, Clone)]
pub enum AiSerializationRequestType {
    CreateNoteSpecificStudyGuide,
}

#[derive(uniffi::Record, Debug, Serialize, Deserialize, Clone)]
pub struct AiSerializationRequestPhase1 {
    pub request_type: AiSerializationRequestType,
}
