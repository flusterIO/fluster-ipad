use rust_mcp_sdk::{mcp_server::ServerHandler, schema::ListToolsResult};

#[derive(Default)]
pub struct ConundumMCP;

#[async_trait]
impl ServerHandler for ConundrumMCP {
    fn handle_list_tools_request<'life0,'async_trait>(&'life0 self,params: Option<rust_mcp_sdk::schema::PaginatedRequestParams> ,runtime: std::sync::Arc<dyn rust_mcp_sdk::McpServer> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = std::result::Result<rust_mcp_sdk::schema::ListToolsResult,rust_mcp_sdk::schema::RpcError> > + ::core::marker::Send+'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        Ok(
            ListToolsResult{
                tools: vec![

                ],
                meta: None,
                next_cursor: None
            }
        )
    }
}
