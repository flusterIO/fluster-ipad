use arrow_schema::{Field, Fields};

pub fn workspace_relative_path_field(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
    Field::new(field_key.to_string(),
               arrow_schema::DataType::Struct(Fields::from(vec![Field::new("workspace_path",
                                                                           arrow_schema::DataType::Utf8,
                                                                           false),
                                                                Field::new("relative_path",
                                                                           arrow_schema::DataType::Utf8,
                                                                           false),])),
               nullable)
}
