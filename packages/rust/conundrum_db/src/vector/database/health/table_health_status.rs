use conundrum::ecosystem::db::tables::DatabaseTable;

pub struct TableHealthStatus {
    pub table: DatabaseTable,
    pub exists: bool,
    /// True if the table is a temporary vector table. It's possible for a table
    /// to be vector based and _not_ be temporary, such as the tools index,
    /// but this will be true if the table may logically be missing while
    /// the app is still in a good state.
    pub is_temp_vector: bool,
}
