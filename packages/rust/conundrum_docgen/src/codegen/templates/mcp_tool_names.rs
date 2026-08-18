use askama::Template;
use conundrum::{
    ai::models::agent::agent_primary_task::AgentPrimaryTask, ai::models::tool::mcp_tool_name::MCPToolName,
    lang::lib::ui::ui_types::emphasis::emphasis_model::Emphasis,
    output::html::web_specific_traits::css_value_representable::CSSVariablePairRepresentable,
};
use strum::IntoEnumIterator;

use crate::traits::DocGenTemplate;

#[derive(Template)]
#[template(path = "typescript/mcp_tool_names.txt", ext = "jinja")]
pub struct MCPToolNameList {}

impl MCPToolNameList {
    pub fn all_tool_names(&self) -> Vec<MCPToolName> {
        MCPToolName::iter().collect::<Vec<MCPToolName>>()
    }

    pub fn all_ai_tasks(&self) -> Vec<AgentPrimaryTask> {
        AgentPrimaryTask::iter().collect::<Vec<AgentPrimaryTask>>()
    }
}

impl DocGenTemplate for MCPToolNameList {
    fn descriptive_label() -> String {
        String::from("Emphasis variable match")
    }

    fn gather_data() -> Self {
        Self {}
    }
}
