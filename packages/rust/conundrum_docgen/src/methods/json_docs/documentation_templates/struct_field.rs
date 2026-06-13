use crate::methods::json_docs::documentation_templates::struct_general_field::StructGeneralField;

#[derive(askama::Template)]
#[template(ext = "jinja")]
pub enum StructField {
    #[template(path = "markdown/documentation/from_rust/struct/struct_fields/string.txt")]
    String(StructGeneralField<String>),
    #[template(path = "markdown/documentation/from_rust/struct/struct_fields/number.txt")]
    Float(StructGeneralField<f64>),
    #[template(path = "markdown/documentation/from_rust/struct/struct_fields/number.txt")]
    Int(StructGeneralField<i64>),
}
