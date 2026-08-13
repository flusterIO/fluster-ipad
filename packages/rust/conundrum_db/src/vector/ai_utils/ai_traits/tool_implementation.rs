use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use schemars::SchemaGenerator;

use crate::vector::models::ai::tool::{mcp_tool_definition::MCPToolDefinition, mcp_tool_name::MCPToolName};

pub trait ToolImplementation<ResultType, ErrorType = DatabaseError> {
    fn name() -> MCPToolName;
    // fn definition(schema_generator: &mut SchemaGenerator) ->
    // Result<MCPToolDefinition, ErrorType>;
    async fn execute() -> Result<ResultType, ErrorType>;
}
