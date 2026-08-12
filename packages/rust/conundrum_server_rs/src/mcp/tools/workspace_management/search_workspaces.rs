use rust_mcp_sdk::macros::mcp_tool;

/// Search through the user's workspaces at the top most level. This will not
// return individual
/// files, but workspaces which can be used for further searching.
#[mcp_tool(name = "search_user_workspaces", description = "Search the user's workspaces")]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, schemars::JsonSchema)]
pub struct SearchWorkspacesTool {
    pub fs_root: Option<String>,
}
