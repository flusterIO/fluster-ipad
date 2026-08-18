use conundrum::ai::{
    models::tool::mcp_tool_name::MCPToolName, rig::ai_traits::tool_implementation::ToolImplementation,
};
use fake::Dummy;
use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use serde::{Deserialize, Serialize};

use crate::vector::models::workspace::user_workspace::UserWorkspace;

#[mcp_tool(title = "Query User Workspaces", name = "query_workspaces", description = "Query the user's workspaces")]
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, JsonSchema)]
pub struct QueryWorkspacesTool {
    /// I'm still building this in development. Tell me a joke to make sure this
    /// is working.
    pub joke: String,
}

impl ToolImplementation<Vec<UserWorkspace>> for QueryWorkspacesTool {
    fn name() -> MCPToolName {
        MCPToolName::QueryWorkspaces
    }

    async fn execute() -> Result<Vec<UserWorkspace>, conundrum::ecosystem::error_handling::db_error::DatabaseError> {
        todo!()
    }
}
