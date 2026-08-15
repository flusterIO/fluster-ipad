use rig::providers::ollama::{Client as OllamaClient, OllamaApiKey};

use crate::rig::{rig_client_local::RigClientLocal, rig_client_remote::RigClientRemote};

pub enum RigClient {
    Local(RigClientLocal),
    Remote(RigClientRemote),
}
