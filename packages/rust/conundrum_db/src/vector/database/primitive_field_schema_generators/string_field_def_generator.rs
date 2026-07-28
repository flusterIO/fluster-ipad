use conundrum::ecosystem::db::tables::DatabaseTable;

pub fn string_field_definition(field_name: &'static str, table: &DatabaseTable) -> String {
    format!("DEFINE FIELD IF NOT EXISTS {} ON {} type string;", field_name, table)
}

pub fn optional_string_field_definition(field_name: &'static str, table: &DatabaseTable) -> String {
    format!("DEFINE FIELD IF NOT EXISTS {} ON {} TYPE option<string>;", field_name, table)
}

pub fn optional_float_field_definition(field_name: &'static str, table: &DatabaseTable) -> String {
    format!("DEFINE FIELD IF NOT EXISTS {} ON {} TYPE option<float>;", field_name, table)
}

pub fn boolean_field_definition(field_name: &'static str, table: &DatabaseTable) -> String {
    format!("DEFINE FIELD IF NOT EXISTS {} ON {} type bool;", field_name, table)
}
