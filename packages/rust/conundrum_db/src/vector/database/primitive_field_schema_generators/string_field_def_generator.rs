use std::fmt::Display;

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

pub fn optional_clamped_float_field_definition<T: Display>(field_name: &'static str,
                                                           table: &DatabaseTable,
                                                           min: T,
                                                           max: T)
                                                           -> String {
    format!("DEFINE FIELD IF NOT EXISTS {} ON {} TYPE option<float> ASSERT $value in {}..={};",
            field_name, table, min, max)
}

pub fn boolean_field_definition(field_name: &'static str, table: &DatabaseTable) -> String {
    format!("DEFINE FIELD IF NOT EXISTS {} ON {} type bool;", field_name, table)
}

pub fn unsigned_int_field_definition(field_name: &'static str, table: &DatabaseTable) -> String {
    format!("DEFINE FIELD IF NOT EXISTS {} ON {} type int ASSERT $value >= 0;", field_name, table)
}
