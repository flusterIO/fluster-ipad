use std::hash::Hash;

use crate::ecosystem::error_handling::db_error::DatabaseError;
use convert_case::Casing;
use fake::Dummy;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumIter};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Deserialize, Display, EnumIter, EnumCount, PartialEq, Clone, Eq, specta::Type, Dummy)]
pub enum DatabaseTable {
    #[strum(to_string = "ecosystem_log")]
    #[serde(rename = "ecosystem_log")]
    EcosystemLog,
    #[strum(to_string = "tag")]
    #[serde(rename = "tag")]
    Tag,
    #[strum(to_string = "topic")]
    #[serde(rename = "topic")]
    Topic,
    #[strum(to_string = "subject")]
    #[serde(rename = "subject")]
    Subject,
    #[strum(to_string = "cdrm")]
    #[serde(rename = "cdrm")]
    Cdrm,
    #[strum(to_string = "typst")]
    #[serde(rename = "typst")]
    TypstContent,
    #[strum(to_string = "user_workspace")]
    #[serde(rename = "user_workspace")]
    UserWorkspace,
    #[strum(to_string = "workspace_path")]
    #[serde(rename = "workspace_path")]
    WorkspacePath,
    #[strum(to_string = "qa_pair")]
    #[serde(rename = "qa_pair")]
    QAPair,
    #[strum(to_string = "chat_conversation")]
    #[serde(rename = "chat_conversation")]
    ChatConversation,
    #[strum(to_string = "agent_message")]
    #[serde(rename = "agent_message")]
    AgentMessage,
    #[strum(to_string = "system_prompt_message")]
    #[serde(rename = "system_prompt_message")]
    SystemPromptMessage,
    #[strum(to_string = "chat_message")]
    #[serde(rename = "chat_message")]
    UserMessage,
    #[strum(to_string = "reasoning_block")]
    #[serde(rename = "reasoning_block")]
    AgentReasoning,
    #[strum(to_string = "tool_execution")]
    #[serde(rename = "tool_execution")]
    ToolExecution,
    #[strum(to_string = "academic_res_metric")]
    #[serde(rename = "academic_res_metric")]
    AcademicResultMetric,
    #[strum(to_string = "bib_entry")]
    #[serde(rename = "bib_entry")]
    BibEntry,
    #[strum(to_string = "auto_taggable")]
    #[serde(rename = "auto_taggable")]
    AutoTaggable,
    #[strum(to_string = "milestone")]
    #[serde(rename = "milestone")]
    Milestone,
    #[strum(to_string = "assignment")]
    #[serde(rename = "assignment")]
    Assignment,
    #[strum(to_string = "assignment_tag")]
    #[serde(rename = "assignment_tag")]
    AssignmentTag,
    #[strum(to_string = "assignment_topic")]
    #[serde(rename = "assignment_topic")]
    AssignmentTopic,
    #[strum(to_string = "assignment_subject")]
    #[serde(rename = "assignment_subject")]
    AssignmentSubject,
    #[strum(to_string = "agent_description")]
    #[serde(rename = "agent_description")]
    AgentDescription,
    #[strum(to_string = "numeric_academic_res_metric")]
    #[serde(rename = "numeric_academic_res_metric")]
    /// Stores just the `AcademicResultMetricKey` and the value.
    NumericAcademicResultMetric,
    #[strum(to_string = "rational_academic_res_metric")]
    #[serde(rename = "rational_academic_res_metric")]
    RationalScoreAcademicResultMetric,
    #[strum(to_string = "custom_academic_res_metric")]
    #[serde(rename = "custom_academic_res_metric")]
    CustomAcademicResultMetric,
    #[strum(to_string = "git_repository")]
    #[serde(rename = "git_repository")]
    GitRepository,
    #[strum(to_string = "keyboard_shortcut")]
    #[serde(rename = "keyboard_shortcut")]
    KeyboardShortcut,
    /// --- 'Joining' tables ---
    #[strum(to_string = "workspace_repository")]
    #[serde(rename = "workspace_repository")]
    WorkspaceRepository,
    #[strum(to_string = "milestone_alarm")]
    #[serde(rename = "milestone_alarm")]
    MilestoneAlarm,
    /// ---- Vectors ----
    #[strum(to_string = "cdrm_vec")]
    #[serde(rename = "cdrm_vec")]
    MarkdownChunk,
    #[strum(to_string = "mcp_tool")]
    #[serde(rename = "mcp_tool")]
    MCPToolRecord,
    #[strum(to_string = "documentation_chunk")]
    #[serde(rename = "documentation_chunk")]
    DocumentationChunk,
}

impl Hash for DatabaseTable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl DatabaseTable {
    pub fn all_temporary_tables() -> Vec<Self> {
        vec![Self::MarkdownChunk, Self::DocumentationChunk]
    }

    /// Deprecated. Surreal was a hugeeee mistake.
    pub fn is_schemafull(&self) -> bool {
        true
    }

    /// TODO: Move this to a macro or to a build-time calculation
    pub fn all_permanent_tables() -> Vec<Self> {
        let mut items = Vec::new();
        let temp_tables = Self::all_temporary_tables();
        for table in DatabaseTable::iter() {
            if !temp_tables.contains(&table) {
                items.push(table.clone());
            }
        }
        items
    }

    pub fn is_temporary_vector_table(&self) -> bool {
        match self {
            Self::MarkdownChunk => true,
            _ => false,
        }
    }

    /// Returns a name of the struct stored in the table for displaying user
    /// facing information.
    pub fn to_model_name(&self) -> String {
        match self {
            Self::Cdrm => String::from("Conundrum"),
            Self::QAPair => String::from("FlashCard"),
            Self::MCPToolRecord => String::from("MCP Tool"),
            _ => self.to_string().to_case(convert_case::Case::Title),
        }
    }
}

impl TryFrom<String> for DatabaseTable {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for s in Self::iter() {
            if s.to_string() == value {
                return Ok(s);
            }
        }
        return Err(DatabaseError::SerializationError);
    }
}
