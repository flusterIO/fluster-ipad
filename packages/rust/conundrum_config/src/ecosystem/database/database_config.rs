use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ## To-Do
// - [ ] Add secure auth, especially in connection with the server package.
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct DatabaseConfig {}
