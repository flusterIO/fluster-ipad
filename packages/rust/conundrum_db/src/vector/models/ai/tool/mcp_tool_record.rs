use std::sync::Arc;

use crate::vector::models::vector::vector::DBVector;
use conundrum::ecosystem::db::db_traits::{
    db_entity::{DBEntity, DBSchema},
    db_field::DatabaseField,
};
use fake::Dummy;
use rust_mcp_sdk::schema::Tool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct MCPToolRecord {
    pub name: String,
    pub description: String,
    pub input_schema_json: String,
    pub vector: DBVector,
}

impl<'a> DBSchema<'a> for MCPToolRecord {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(String::field_definition("name", false)),
                Arc::new(String::field_definition("description", false)),
                Arc::new(String::field_definition("input_schema_json", false)),
                Arc::new(DBVector::field_definition(false))])
    }
}

impl<'a> DBEntity<'a> for MCPToolRecord {
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

    fn primary_value(&self) -> String {
        self.name.to_string()
    }
}

impl MCPToolRecord {
    pub fn from_tool_and_embedding(tool: Tool, input_schema_json: String, vec: Vec<f64>) -> Self {
        MCPToolRecord { name: tool.name.clone(),
                        description: tool.description.unwrap_or_default(),
                        input_schema_json,
                        vector: DBVector(vec) }
    }
}
