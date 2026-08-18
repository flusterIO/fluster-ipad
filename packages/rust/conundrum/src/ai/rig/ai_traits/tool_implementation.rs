use crate::{ai::models::tool::mcp_tool_name::MCPToolName, ecosystem::error_handling::db_error::DatabaseError};

pub trait ToolImplementation<ResultType, ErrorType = DatabaseError> {
    fn name() -> MCPToolName;
    // fn definition(schema_generator: &mut SchemaGenerator) ->
    // Result<MCPToolDefinition, ErrorType>;
    async fn execute() -> Result<ResultType, ErrorType>;
}
