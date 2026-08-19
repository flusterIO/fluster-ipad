use crate::{
    ai::{
        ai_constants::{
            BASE_TEMPERATURE_AGENT, BASE_TEMPERATURE_CHAT, BASE_TEMPERATURE_CLASSIFICATION,
            BASE_TEMPERATURE_CODE_GENERATION, BASE_TEMPERATURE_CODE_TRANSFORMATION,
            BASE_TEMPERATURE_CREATIVE_GENERATION, BASE_TEMPERATURE_EXTRACTION, BASE_TEMPERATURE_NOTE_CREATION,
            BASE_TEMPERATURE_QUESTION_ANSWERING, BASE_TEMPERATURE_STRUCTURED_GENERATION,
            BASE_TEMPERATURE_TEXT_SUMMARIZATION, BASE_TEMPERATURE_TOOL_CALLING,
        },
        models::agent::agent_description::AgentDescription,
    },
    ecosystem::{db::db_traits::db_field::DatabaseField, error_handling::db_error::DatabaseError},
};
use fake::Dummy;
use strum::IntoEnumIterator;

#[derive(serde::Serialize,
           serde::Deserialize,
           Clone,
           Debug,
           specta::Type,
           Dummy,
           strum_macros::EnumIter,
           strum_macros::EnumString,
           strum_macros::Display)]
#[strum(serialize_all = "kebab-case")]
#[serde(try_from = "String", into = "String", rename_all = "kebab-case")]
pub enum AgentPrimaryTask {
    Embedding,
    Classification,
    Extraction,
    StructuredGeneration,
    CodeGeneration,
    CodeTransformation,
    Summarization,
    QuestionAnswering,
    CreativeGeneration,
    ToolCalling,
    Agent,
}

impl TryFrom<String> for AgentPrimaryTask {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for k in AgentPrimaryTask::iter() {
            if k.to_string() == value {
                return Ok(k);
            }
        }
        return Err(DatabaseError::SerializationError);
    }
}

impl Into<String> for AgentPrimaryTask {
    fn into(self) -> String {
        self.to_string()
    }
}

impl DatabaseField for AgentPrimaryTask {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}

impl Into<AgentDescription> for AgentPrimaryTask {
    fn into(self) -> AgentDescription {
        match self {
            Self::Embedding => AgentDescription { max_tokens: None,
                                                  allow_tools: false,
                                                  ..Default::default() },

            Self::Classification => AgentDescription { max_tokens: Some(128),
                                                       allow_tools: false,
                                                       ..Default::default() },

            Self::Extraction => AgentDescription { max_tokens: Some(1024),
                                                   allow_tools: false,
                                                   ..Default::default() },

            Self::StructuredGeneration => AgentDescription { max_tokens: Some(2048),
                                                             allow_tools: false,
                                                             ..Default::default() },

            Self::CodeGeneration => AgentDescription { max_tokens: Some(8192),
                                                       allow_tools: true,
                                                       ..Default::default() },

            Self::CodeTransformation => AgentDescription { max_tokens: Some(8192),
                                                           allow_tools: true,
                                                           ..Default::default() },

            Self::Summarization => AgentDescription { max_tokens: Some(2048),
                                                      allow_tools: false,
                                                      ..Default::default() },

            Self::QuestionAnswering => AgentDescription { max_tokens: Some(4096),
                                                          allow_tools: true,
                                                          ..Default::default() },

            Self::CreativeGeneration => AgentDescription { max_tokens: Some(4096),
                                                           allow_tools: false,
                                                           ..Default::default() },

            Self::ToolCalling => AgentDescription { max_tokens: Some(2048),
                                                    allow_tools: true,
                                                    ..Default::default() },

            Self::Agent => AgentDescription { max_tokens: Some(8192),
                                              allow_tools: true,
                                              ..Default::default() },
        }
    }
}

impl AgentPrimaryTask {
    pub fn to_base_temperature(&self) -> f64 {
        match self {
            Self::Agent => BASE_TEMPERATURE_AGENT as f64,
            Self::Summarization => BASE_TEMPERATURE_TEXT_SUMMARIZATION as f64,
            Self::QuestionAnswering => BASE_TEMPERATURE_QUESTION_ANSWERING as f64,
            Self::CreativeGeneration => BASE_TEMPERATURE_CREATIVE_GENERATION as f64,
            Self::Classification => BASE_TEMPERATURE_CLASSIFICATION as f64,
            Self::CodeTransformation => BASE_TEMPERATURE_CODE_TRANSFORMATION as f64,
            Self::CodeGeneration => BASE_TEMPERATURE_CODE_GENERATION as f64,
            Self::Extraction => BASE_TEMPERATURE_EXTRACTION as f64,
            Self::ToolCalling => BASE_TEMPERATURE_TOOL_CALLING as f64,
            Self::StructuredGeneration => BASE_TEMPERATURE_STRUCTURED_GENERATION as f64,
            Self::Embedding => 0.0,
        }
    }
}
