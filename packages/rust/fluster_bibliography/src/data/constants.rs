use serde::{Deserialize, Serialize};

#[derive(uniffi::Enum, Serialize, Deserialize)]
pub enum RenderMethod {
    Html,
    Plaintext,
}
