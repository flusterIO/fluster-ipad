use rust_mcp_sdk::schema::Tool;

use crate::mcp::tools::workspace_management::search_workspaces::SearchWorkspacesTool;

pub struct ToolList(pub Vec<Tool>);

impl ToolList {
    /// Returns all tools on the entire server.
    pub fn all_tools() -> ToolList {
        ToolList(vec![SearchWorkspacesTool::tool()])
    }
}
