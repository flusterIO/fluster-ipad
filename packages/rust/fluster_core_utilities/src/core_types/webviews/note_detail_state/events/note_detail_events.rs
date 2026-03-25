use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, serde::Serialize, serde::Deserialize, strum_macros::Display)]
pub enum NoteDetailEvents {
    /// Ask the AI to generate a summary of the currently focused note.
    #[serde(rename = "send-gen-summary-req")]
    #[strum(to_string = "send-gen-summary-req")]
    SendGenerateSummaryRequest,
}
