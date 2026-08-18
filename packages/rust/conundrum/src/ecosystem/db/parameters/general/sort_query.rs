use lancedb::query::ColumnOrdering;

use crate::ecosystem::db::parameters::general::sort_order::SortOrder;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct SortQuery {
    pub column: String,
    pub order: SortOrder,
}

impl SortQuery {
    pub fn to_column_ordering(&self) -> ColumnOrdering {
        self.order.to_lancedb(self.column.clone())
    }
}

pub struct SortQueryList(pub Vec<SortQuery>);

impl SortQueryList {
    pub fn to_column_orderings(&self) -> Vec<ColumnOrdering> {
        self.0.iter().map(|x| x.to_column_ordering()).collect()
    }
}
