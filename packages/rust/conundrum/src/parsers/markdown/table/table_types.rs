use std::fmt::Display;

use askama::FastWritable;
use dashmap::DashMap;

#[derive(Clone, Copy, Default)]
pub enum TableCellAlignment {
    Left,
    Right,
    Center,
    #[default]
    Default,
}

impl Display for TableCellAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_class())
    }
}

impl FastWritable for TableCellAlignment {
    fn write_into<W: core::fmt::Write + ?Sized>(&self,
                                                dest: &mut W,
                                                values: &dyn askama::Values)
                                                -> askama::Result<()> {
        self.as_class().write_into(dest, values)
    }
}

impl TableCellAlignment {
    pub fn as_class(&self) -> String {
        match self {
            Self::Left => "text-left".to_string(),
            Self::Right => "text-right".to_string(),
            Self::Center => "text-center".to_string(),
            Self::Default => "text-left".to_string(),
        }
    }
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
            *res.value()
        } else {
            TableCellAlignment::default()
        }
    }
}
