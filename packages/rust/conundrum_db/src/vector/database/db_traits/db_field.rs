use lancedb::arrow::arrow_schema::Field;

pub trait DatabaseField<T, FieldDefinitions = Field> {
    fn field_definition(field_key: &'static str, nullable: bool) -> FieldDefinitions;
    fn to_db_representation(&self) -> T;
}

/// Exactly the same as teh DatabaseField but wihhout the extra parameters that
/// aren't needed for field definitions that are repeated frequently, and in
/// fact might muddy up the code base leaving them around.
pub trait RepeatedDatabaseField<FieldDefinitions = Field> {
    fn field_definition() -> FieldDefinitions;
}
