use lancedb::arrow::arrow_schema::Field;

pub trait DatabaseField {
    fn field_definition(field_key: &'static str, nullable: bool) -> Field;
}
