use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize, Deserialize, uniffi::Record)]
pub struct PaginationParams {
    pub per_page: u64,
    pub page: u64,
}
