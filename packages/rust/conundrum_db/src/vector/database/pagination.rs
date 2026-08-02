use conundrum::ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaginationParams {
    pub per_page: usize,
    pub page: usize,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self { per_page: 10,
               page: 1 }
    }
}

impl PaginationParams {
    pub fn validate(&self) -> DatabaseResult<()> {
        if (self.page <= 0) {
            Err(DatabaseError::InvalidPagination)
        } else {
            Ok(())
        }
    }
}
