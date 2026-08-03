use serde::{Deserialize, Serialize};
use specta::Type;

use crate::vector::database::pagination::PaginationParams;

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct PredicateQueryParams {
    pub predicate: Option<String>,
    pub pagination: Option<PaginationParams>,
}
