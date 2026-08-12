use arrow_array::builder::FixedSizeBinaryBuilder;
use arrow_schema::{DataType, Field};

pub fn two_required_id_fields(field_key_one: &'static str, field_key_two: &'static str) -> (Field, Field) {
    (Field::new(field_key_one.to_string(), DataType::Utf8, false),
     Field::new(field_key_two.to_string(), DataType::Utf8, false))
}
