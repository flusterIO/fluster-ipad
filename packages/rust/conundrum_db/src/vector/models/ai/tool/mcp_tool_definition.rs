use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MCPToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}
