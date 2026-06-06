use serde::{Deserialize, Serialize};

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, uniffi::Record)]
pub struct ParseConundrumContentRequest {
    /// The cdrm content.
    pub content: String,
    /// The DOM id which the parsed content should be appended to. This will
    /// replace that element's children.
    #[allow(non_snake_case)]
    pub DOMId: String,
}
