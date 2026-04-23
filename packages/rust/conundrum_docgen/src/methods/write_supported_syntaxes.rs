use std::fmt::Display;

use askama::{Result, Template};
use conundrum::{
    lang::runtime::queries::code::get_supported_syntaxes::get_supported_syntaxes,
    testing::get_workspace_root::get_workspace_root,
};
use tabled::{Table, Tabled, settings::Style};

#[derive(Clone, tabled::Tabled)]
pub struct SupportedThemeEnumVariant {
    pub lang: String,
    #[tabled(format("{:?}", self.extensions))]
    pub extensions: Vec<String>,
}

impl SupportedThemeEnumVariant {
    pub fn format_name(&self) -> String {
        let mut l = self.lang.clone();
        l = l.replace(" ", "_");
        l = l.replace("(", "");
        l = l.replace(")", "");
        l = l.replace("-", "_");
        l = l.replace("++", "pp");
        l = l.replace("/", "_");
        l = l.replace("#", "Sharp");
        l = l.replace(".", "Dot");
        let (leading_char, rest) = l.split_at(1);
        format!("{}{}", leading_char.to_ascii_uppercase(), rest)
    }
}

#[derive(Template)]
#[template(ext = "txt", escape = "none", path = "markdown/in_content/supported-syntax-docs.txt")]
struct SupportedSyntaxDocumentationTempl {
    pub table_string: String,
}

pub fn write_supported_syntaxes() -> Result<(), std::io::Error> {
    let items = get_supported_syntaxes().iter()
                                        .map(|item| SupportedThemeEnumVariant { lang: item.lang.clone(),
                                                                                extensions: item.extensions.clone() })
                                        .collect::<Vec<SupportedThemeEnumVariant>>();

    let mut table = Table::new(&items);
    let markdown_table = table.with(Style::markdown());

    let table_string = markdown_table.to_string();

    let templ = SupportedSyntaxDocumentationTempl { table_string };

    let docs_content = templ.render().expect("Renders supported syntax documentation without throwing an error.");

    let output_path =
        std::path::Path::new(&get_workspace_root()).join("docs").join("in_content_docs").join("supported-syntaxes.mdx");

    std::fs::write(output_path, docs_content)?;
    Ok(())
}
