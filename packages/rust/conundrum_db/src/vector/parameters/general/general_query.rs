use fake::Dummy;
use lancedb::query::ColumnOrdering;
use serde::{Deserialize, Serialize};

use crate::vector::{database::pagination::PaginationParams, parameters::general::sort_query::SortQuery};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct GeneralQuery<PredicateType = Option<String>> {
    pub predicate: PredicateType,
    pub pagination: PaginationParams,
    pub sort: Option<Vec<SortQuery>>,
}

impl GeneralQuery {
    pub fn to_column_orderings(&self) -> Option<Vec<ColumnOrdering>> {
        self.sort.as_ref().cloned().map(|x| x.iter().map(|x| x.to_column_ordering()).collect())
    }
}
