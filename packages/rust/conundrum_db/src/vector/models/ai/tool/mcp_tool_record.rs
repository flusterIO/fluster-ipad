use std::sync::Arc;

use conundrum::{
    ai::models::{chat::vector::vector_model::DBVector, tool::mcp_tool_name::MCPToolName},
    ecosystem::db::db_traits::{
        db_entity::{DBEntity, DBSchema},
        db_field::DatabaseField,
    },
};
use fake::Dummy;
use rust_mcp_sdk::schema::Tool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct MCPToolRecord {
    pub name: MCPToolName,
    pub description: String,
    pub input_schema_json: String,
    pub local_vector: DBVector,
    pub remote_vector: Option<DBVector>,
}

impl<'a> DBSchema<'a> for MCPToolRecord {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(String::field_definition("name", false)),
                Arc::new(String::field_definition("description", false)),
                Arc::new(String::field_definition("input_schema_json", false)),
                Arc::new(DBVector::field_definition("local_vector", true)),
                Arc::new(DBVector::field_definition("remote_vector", true)),])
    }
}

impl<'a> DBEntity<'a, MCPToolName> for MCPToolRecord {
    type PartialUpdateType = MCPToolRecord;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        conundrum::ecosystem::db::tables::DatabaseTable::MCPToolRecord
    }

    fn merge_keys() -> &'static [&'static str] {
        &["name"]
    }

    fn primary_key() -> &'static str {
        "name"
    }

    fn primary_value(&self) -> MCPToolName {
        self.name.clone()
    }

    fn set_primary_value(&mut self, value: MCPToolName) {
        self.name = value.clone()
    }
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
