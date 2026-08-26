use std::sync::Arc;

use rmcp::{
    RoleServer, ServerHandler,
    handler::server::wrapper::Parameters,
    model::{
        CallToolRequestParams, CallToolResponse, CallToolResult, ErrorData, Implementation, ListToolsResult,
        PaginatedRequestParams, PingRequestMethod, ServerCapabilities, ServerInfo, TextContent, Tool,
    },
    service::RequestContext,
    tool,
};

use schemars::JsonSchema;
use serde::Deserialize;

// ============================================================
// Tool parameters
// ============================================================

#[derive(Debug, Deserialize, JsonSchema)]
pub struct HelloParams {
    /// The name to greet.
    pub name: String,
}

// ============================================================
// MCP server
// ============================================================

#[derive(Clone, Default)]
pub struct ConundrumMcpServer;

// ============================================================
// Tools
// ============================================================

impl ConundrumMcpServer {
    #[tool(name = "hello", description = "Return a greeting for a person.")]
    pub async fn hello(&self, Parameters(params): Parameters<HelloParams>) -> Result<CallToolResult, ErrorData> {
        Ok(CallToolResult::success(vec![rmcp::model::ContentBlock::Text(TextContent::new(format!("Hello, {}!",
                                                                                                 params.name))),]))
    }
}

// ============================================================
// MCP ServerHandler
// ============================================================

impl ServerHandler for ConundrumMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .build(),
        )
        .with_server_info(
            Implementation::new(
                "conundrum-mcp",
                env!("CARGO_PKG_VERSION"),
            )
        )
    }

    async fn list_tools(&self,
                        _request: Option<PaginatedRequestParams>,
                        _context: RequestContext<RoleServer>)
                        -> Result<ListToolsResult, ErrorData> {
        // We'll populate this manually for now.
        //
        // This is intentionally explicit because you asked
        // to avoid #[tool_router].
        //
        // When you add another tool, add its Tool definition here.

        let tool = Tool::new("hello",
                             "Return a greeting for a person.",
                             Arc::new(serde_json::json!({
                                          "type": "object",
                                          "properties": {
                                              "name": {
                                                  "type": "string",
                                                  "description": "The name to greet."
                                              }
                                          },
                                          "required": ["name"],
                                          "additionalProperties": false
                                      }).as_object()
                                        .unwrap()
                                        .clone()));

        Ok(ListToolsResult::with_all_items(vec![tool]))
    }

    async fn call_tool(&self,
                       request: CallToolRequestParams,
                       _context: RequestContext<RoleServer>)
                       -> Result<CallToolResponse, ErrorData> {
        match request.name.as_ref() {
            "hello" => {
                let arguments = request.arguments.unwrap_or_default();

                let params: HelloParams =
                    serde_json::from_value(serde_json::Value::Object(arguments)).map_err(|error| {
                                                                                    ErrorData::invalid_params(
                            error.to_string(),
                            None,
                        )
                                                                                })?;

                let r = self.hello(Parameters(params)).await?;
                Ok(CallToolResponse::Complete(r))
            }

            _ => Err(ErrorData::method_not_found::<PingRequestMethod>()),
        }
    }
}
