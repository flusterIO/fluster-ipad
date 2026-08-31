use std::sync::Arc;

use conundrum::{
    ai::models::{chat::vector::vector_model::DBVector, tool::mcp_tool_name::MCPToolName},
    ecosystem::db::{
        db_traits::{
            db_entity::{DBEntity, DBSchema},
            db_field::DatabaseField,
        },
        tables::DatabaseTable,
    },
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use rust_mcp_sdk::schema::Tool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::MCPToolRecord)]
pub struct MCPToolRecord {
    #[db(primary)]
    pub name: MCPToolName,
    pub description: String,
    pub input_schema_json: String,
    pub local_vector: DBVector,
    pub remote_vector: Option<DBVector>,
}

impl MCPToolRecord {
    pub fn from_tool_and_embedding(tool: Tool,
                                   input_schema_json: String,
                                   local_vec: Vec<f64>,
                                   remote_vec: Option<Vec<f64>>)
                                   -> Self {
        let mcp_tool_name = MCPToolName::try_from(tool.name).expect("Must always unwrap all provided tools.");
        MCPToolRecord { name: mcp_tool_name,
                        description: tool.description.unwrap_or_default(),
                        input_schema_json,
                        local_vector: DBVector(local_vec),
                        remote_vector: remote_vec.map(DBVector) }
    }
}
