use conundrum::ecosystem::db::{db_table_description::DBTableDescription, tables::DatabaseTable};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct TableHealthReport {
    pub exists: bool,
    /// True if the table is a _temporary_ vector table. These tables might be
    /// missing while the app is still in a valid state.
    pub is_temporary_table: bool,
    pub description: DBTableDescription,
}
