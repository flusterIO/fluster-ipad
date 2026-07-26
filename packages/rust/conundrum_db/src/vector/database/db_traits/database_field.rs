use conundrum::ecosystem::db::tables::DatabaseTable;

/// It is not required to implement this trait for all db fields, only for
/// structs or enums that are used repeatedly in the datbaase.
pub trait DatabaseField {
    fn field_definition(field_key: &'static str, table: &DatabaseTable) -> String;
}

pub trait DatabaseRepresentable<T> {
    fn to_database_representation(&self) -> T;
}
