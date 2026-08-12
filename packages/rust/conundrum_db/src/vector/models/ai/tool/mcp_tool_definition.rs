use serde::{Deserialize, Serialize};

use crate::vector::models::ai::tool::mcp_tool_name::MCPToolName;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MCPToolDefinition {
    pub name: MCPToolName,
    pub description: String,
    pub human_description: Option<String>,
    pub input_schema: String,
}
