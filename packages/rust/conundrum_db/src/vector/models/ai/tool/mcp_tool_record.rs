use crate::vector::models::{primitives::db_id::DatabaseId, vector::vector::DBVector};
use rust_mcp_sdk::schema::Tool;

pub struct MCPToolRecord {
    pub name: String,
    pub description: String,
    pub input_schema_json: String,
    pub vector: DBVector,
}

impl MCPToolRecord {
    pub fn from_tool_and_embedding(tool: Tool, input_schema_json: String, vec: Vec<f64>) -> Self {
        // TODO:
        // - [ ] Add an enum of all tool names, and apply that to this function so that
        //   each can be
        // retrieved by that as well, while making sure that there's a unique
        // id-like property. MCPToolRecord { id: Data, name: (),
        // description: (), input_schema_json: (), vector: () }
        MCPToolRecord { name: tool.name.clone(),
                        description: tool.description.unwrap_or_default(),
                        input_schema_json,
                        vector: DBVector(vec) }
    }
}
