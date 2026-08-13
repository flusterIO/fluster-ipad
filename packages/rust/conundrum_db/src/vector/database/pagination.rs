use conundrum::ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult};
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct PaginationParams {
    pub per_page: u32,
    pub page: u32,
}

impl PaginationParams {
    pub fn single() -> Self {
        PaginationParams { per_page: 1,
                           page: 1 }
    }

    pub fn page_minus_one(&self) -> u32 {
        if self.page <= 1 {
            0
        } else {
            self.page - 1
        }
    }

    pub fn to_limit_and_offset(&self) -> (usize, usize) {
        let limit: usize = self.per_page as usize;
        let offset: usize = (self.per_page * (self.page_minus_one())) as usize;
        (limit, offset)
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self { per_page: 10,
               page: 1 }
    }
}
