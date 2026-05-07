use dashmap::DashMap;

pub struct TableCellColumnIndex(u32);

#[derive(Default, Clone, Copy)]
pub enum TableCellAlignment {
    Left,
    Right,
    Center,
    #[default]
    Default,
}

#[derive(Default, Clone)]
pub struct TableCellAlignmentMap(DashMap<usize, TableCellAlignment>);

impl TableCellAlignmentMap {
    pub fn new(data: DashMap<usize, TableCellAlignment>) -> Self {
        Self(data)
    }

    pub fn set_table_col_alignment(&mut self, col_idx: usize, alignment: TableCellAlignment) {
        self.0.insert(col_idx, alignment);
    }

    pub fn get_col_alignment_by_idx(&self, col_idx: &usize) -> TableCellAlignment {
        if let Some(res) = self.0.get(col_idx) {
            res.value().clone()
        } else {
            TableCellAlignment::default()
        }
    }
}
