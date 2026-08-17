use conundrum::ai::models::tool::mcp_tool_name::MCPToolName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MCPToolDefinition {
    pub name: MCPToolName,
    pub description: String,
    pub human_description: Option<String>,
    pub input_schema: String,
}
