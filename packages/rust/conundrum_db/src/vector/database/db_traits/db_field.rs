use lancedb::arrow::arrow_schema::Field;

pub trait DatabaseField<T, FieldDefinitions = Field> {
    fn field_definition(field_key: &'static str, nullable: bool) -> FieldDefinitions;
    fn to_db_representation(&self) -> T;
}
