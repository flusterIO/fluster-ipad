use serde::Serialize;

#[derive(uniffi::Object, Serialize)]
pub struct TagResult {
    pub body: String,
}
