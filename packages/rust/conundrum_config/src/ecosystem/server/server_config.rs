use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// # TO-DO
/// - [ ] Add token param for auth. Never done that in Rust and I don't have
///   access to the internet right now, so I'll come back to it later...
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct ServerConfig {
    pub server_port: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self { server_port: 3005 }
    }
}
