use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct ImageData {
    pub data: Vec<u8>,
    pub id: String,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct MediaState {
    /// A HashMap between an Id and an Object with the necessary image data.
    pub image_data: HashMap<String, ImageData>,
}
