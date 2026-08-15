use std::sync::Arc;

use arrow_schema::Field;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::{database::db_traits::db_field::DatabaseField, models::ai::tool::mcp_tool_name::MCPToolName};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct MCPToolNameList(pub Vec<MCPToolName>);

impl MCPToolNameList {
    pub fn new_empty() -> Self {
        Self(Vec::new())
    }
}

impl DatabaseField for MCPToolNameList {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key.to_string(),
                   arrow_schema::DataType::List(Arc::new(Field::new("item", arrow_schema::DataType::Utf8, true))),
                   nullable)
    }
}
