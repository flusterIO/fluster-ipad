use askama::Template;

use crate::{
    errors::{DocGenError, DocGenResult},
    methods::json_docs::documentation_templates::struct_templ::StructItemTemplate,
};

pub enum RustDocTemplate {
    Struct(StructItemTemplate),
}

impl RustDocTemplate {
    pub fn render(&self) -> DocGenResult<String> {
        match self {
            Self::Struct(s) => s.render().map_err(|e| {
                                             eprintln!("Error: {:#?}", e);
                                             DocGenError::TemplateRenderError(e.to_string())
                                         }),
        }
    }
}
