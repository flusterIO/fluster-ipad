use rig::tool::ToolExecutionError;

use crate::ai::models::tool::{mcp_tool_name::MCPToolName, tool_execution::ToolExecution};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct ToolExecutionPartial {
    pub tool_name: MCPToolName,
    pub args: serde_json::Value,
}
