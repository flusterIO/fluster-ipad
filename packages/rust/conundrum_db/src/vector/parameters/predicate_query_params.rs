use conundrum::ecosystem::db::parameters::general::pagination::PaginationParams;
use serde::{Deserialize, Serialize};
use specta::Type;

/// Deprecated in favor of `GeneralQueryParams`
#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct PredicateQueryParams {
    pub predicate: Option<String>,
    pub pagination: Option<PaginationParams>,
}
