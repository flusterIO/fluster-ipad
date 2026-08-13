use rust_mcp_sdk::schema::Tool;

use crate::vector::tools::workspace::query_workspaces::QueryWorkspacesTool;

pub struct ToolDefinitionList(pub Vec<Tool>);

impl ToolDefinitionList {
    pub fn new_all_tools() -> Self {
        ToolDefinitionList(vec![QueryWorkspacesTool::tool()])
    }
}
