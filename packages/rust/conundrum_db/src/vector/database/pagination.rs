use conundrum::ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
pub struct PaginationParams {
    pub per_page: usize,
    pub page: usize,
}

impl PaginationParams {
    pub fn single() -> Self {
        PaginationParams { per_page: 1,
                           page: 1 }
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self { per_page: 10,
               page: 1 }
    }
}
