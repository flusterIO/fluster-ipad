use lancedb::arrow::arrow_schema::Field;

pub trait DatabaseField<FieldDefinitions = Field> {
    fn field_definition(field_key: &'static str, nullable: bool) -> FieldDefinitions;
}

/// Identical to DatabaseField, but larger... for things like LongUtf8 and
/// LargeBinary or BigBinary or whatever it's called.
pub trait DatabaseFieldLarge<FieldDefinitions = Field> {
    fn field_definition_large(field_key: &'static str, nullable: bool) -> FieldDefinitions;
}

pub trait DatabaseFieldRepresentation<T> {
    fn to_db_representation(&self) -> T;
}

/// Deprecated in favor of a single method trait.
/// Exactly the same as teh DatabaseField but wihhout the extra parameters that
/// aren't needed for field definitions that are repeated frequently, and in
/// fact might muddy up the code base leaving them around.
pub trait RepeatedDatabaseField<FieldDefinitions = Field> {
    fn field_definition() -> FieldDefinitions;
}
