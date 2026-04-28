use serde::Serialize;

#[typeshare::typeshare]
#[derive(Serialize, uniffi::Enum, strum_macros::Display)]
pub enum ConundrumWebEvents {
    /// Useful for things like resizable pains, to more easily emit your own
    /// resize event that Conundrum will handle to update the components
    /// accordingly.
    #[serde(rename = "cdrm-manual-resize")]
    #[strum(to_string = "cdrm-manual-resize")]
    ManualResize,
    #[serde(rename = "cdrm-content-loaded")]
    #[strum(to_string = "cdrm-content-loaded")]
    CdrmContentLoaded,
}
