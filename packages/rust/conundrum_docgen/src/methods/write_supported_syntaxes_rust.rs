use askama::Template;
use conundrum::lang::runtime::queries::code::get_supported_syntaxes::get_supported_syntaxes;

use crate::{
    methods::write_supported_syntaxes::SupportedThemeEnumVariant,
    workspace_utils::get_workspace_root_duplicate::get_workspace_root,
};

#[derive(Template)]
#[template(ext = "txt", escape = "none", path = "rust/supported-syntaxes.txt")]
pub struct SupportedSyntaxesRustTemplate {
    pub items: Vec<SupportedThemeEnumVariant>,
}

impl SupportedSyntaxesRustTemplate {
    pub fn items_with_extensions(&self) -> Vec<SupportedThemeEnumVariant> {
        self.items.iter().filter(|x| !x.extensions.is_empty()).cloned().collect()
    }
}

pub fn write_supported_syntaxes_rust() {
    let items =
        get_supported_syntaxes().iter()
                                .map(|item| SupportedThemeEnumVariant::new(item.lang.clone(), item.extensions.clone()))
                                .collect::<Vec<SupportedThemeEnumVariant>>();
    let templ = SupportedSyntaxesRustTemplate { items };
    let rendered = templ.render().expect("Renders supported syntaxes rust template without throwing an error.");
    let output_path = std::path::Path::new(&get_workspace_root()).join("packages")
                                                                 .join("rust")
                                                                 .join("conundrum")
                                                                 .join("src")
                                                                 .join("parsers")
                                                                 .join("markdown")
                                                                 .join("code_block")
                                                                 .join("supported_languages.rs");
    if let Err(err) = std::fs::write(output_path, rendered) {
        eprintln!("Error: {:#?}", err);
    }
}
