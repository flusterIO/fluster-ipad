use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Deperecated unless we have to lift things up. Use the one in the database
/// package.
#[typeshare]
#[derive(Debug, Serialize, Deserialize, uniffi::Record)]
pub struct PaginationParams {
    pub per_page: u32,
    pub page: u32,
}

impl PaginationParams {
    pub fn page_minus_one(&self) -> u32 {
        if self.page <= 1 {
            0
        } else {
            self.page - 1
        }
    }
}
