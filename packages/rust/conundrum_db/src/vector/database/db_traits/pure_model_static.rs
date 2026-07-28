use conundrum::ecosystem::db::tables::DatabaseTable;

pub trait PureModelStaticMethods {
    fn table() -> DatabaseTable;
    fn schema() -> String;
    /// Returns an optional string that will be ran during initialization to set
    /// any indices on the table associated with Self.
    fn db_index_definitions() -> Option<String> {
        None
    }
    /// Returns the strings defining the schema of the relationships used by
    /// this model.
    fn relation_definitions() -> Option<String> {
        None
    }
}
