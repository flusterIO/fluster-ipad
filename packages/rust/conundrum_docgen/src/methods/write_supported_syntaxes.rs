use std::fmt::Display;

use askama::{Result, Template};
use conundrum::lang::runtime::queries::code::get_supported_syntaxes::get_supported_syntaxes;
use tabled::{Table, settings::Style};

use crate::workspace_utils::get_workspace_root_duplicate::get_workspace_root;

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

    pub fn get_formated_key(&self) -> String {
        let name = self.format_name();
        name.to_lowercase().replace("_", "-")
    }

    pub fn new(lang: String, extensions: Vec<String>) -> Self {
        let mut item = SupportedThemeEnumVariant { lang,
                                                   extensions };
        let formatted_key = item.get_formated_key();
        if !item.extensions.contains(&formatted_key) {
            item.extensions.push(formatted_key);
        }
        item
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
