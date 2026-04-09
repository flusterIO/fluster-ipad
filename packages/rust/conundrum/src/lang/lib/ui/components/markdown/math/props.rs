/// The 'math' is embedded in `.jsx` as a child.
#[typeshare::typeshare]
#[derive(uniffi::Record, serde::Serialize)]
pub struct MathData {
    pub display: bool,
    /// The `idx` field will only be set for equations where display=true, for
    /// obvious reasons.
    pub idx: Option<u32>,
    /// An optional user provided id, only applicable for block-level math.
    pub id: Option<String>,
    /// The primary math content.
    pub content: String,
}
