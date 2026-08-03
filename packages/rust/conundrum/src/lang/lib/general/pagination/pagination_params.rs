use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Deperecated unless we have to lift things up. Use the one in the database
/// package.
#[typeshare]
#[derive(Debug, Serialize, Deserialize, uniffi::Record)]
pub struct PaginationParams {
    pub per_page: u64,
    pub page: u64,
}
