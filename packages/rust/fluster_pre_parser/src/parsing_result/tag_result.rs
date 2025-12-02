use serde::Serialize;

#[derive(uniffi::Object, Serialize)]
pub struct TagResult {
    pub body: String,
}

impl TagResult {
    pub fn new(body: String) -> TagResult {
        TagResult { body }
    }
}
