use conundrum::ecosystem::db::tables::DatabaseTable;

pub trait EnumDBModelVariantMethods {
    fn table(&self) -> DatabaseTable;
    fn schema(&self) -> String;
}
