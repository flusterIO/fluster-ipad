use conundrum::ecosystem::db::tables::DatabaseTable;

pub trait PureModelStaticMethods {
    fn table() -> DatabaseTable;
    fn schema() -> String;
    /// Returns an optional string that will be ran during initialization to set
    /// any indices on the table associated with Self.
    fn database_indices() -> Option<String> {
        None
    }
}
