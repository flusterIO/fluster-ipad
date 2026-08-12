use async_trait::async_trait;
use rust_mcp_sdk::{
    McpServer,
    macros::{JsonSchema, mcp_tool},
    mcp_server::ServerHandler,
    schema::{CallToolError, CallToolRequestParams, CallToolResult, ListToolsResult, PaginatedRequestParams, RpcError},
};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ConundrumMCP;

#[mcp_tool(name = "hello_world", description = "Say hello to the user.")]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct HelloWorld {}

#[derive(Default)]
pub struct HelloWorldHandler;

#[async_trait]
impl ServerHandler for ConundrumMCP {
    async fn handle_list_tools_request(&self,
                                       _request: Option<PaginatedRequestParams>,
                                       _runtime: std::sync::Arc<dyn McpServer>)
                                       -> Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult { meta: None,
                             next_cursor: None,
                             tools: vec![HelloWorld::tool()] })
    }

    async fn handle_call_tool_request(&self,
                                      params: CallToolRequestParams,
                                      _runtime: std::sync::Arc<dyn McpServer>)
                                      -> Result<CallToolResult, CallToolError> {
        if params.name == "hello_world" {
            Ok(CallToolResult::text_content(vec!["Tudalu mothafuckaaaaaaa".into()]))
        } else {
            Err(CallToolError::unknown_tool(params.name))
        }
    }
}
