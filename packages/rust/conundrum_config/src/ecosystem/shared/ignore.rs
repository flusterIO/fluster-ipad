use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct IgnoreConfig {
    pub respect_gitignore: bool,
}
