#[derive(Default, Clone, Copy)]
pub enum TableCellAlignment {
    Left,
    Right,
    Center,
    #[default]
    Default,
}
