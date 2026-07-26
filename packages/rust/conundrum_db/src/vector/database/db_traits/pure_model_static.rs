use conundrum::ecosystem::db::tables::DatabaseTable;

pub trait PureModelStaticMethods {
    fn table() -> DatabaseTable;
    fn schema() -> String;
}
