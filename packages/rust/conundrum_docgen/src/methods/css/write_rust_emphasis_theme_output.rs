use conundrum::{
    lang::{
        lib::ui::ui_types::emphasis::Emphasis,
        runtime::state::{conundrum_error::ConundrumError, conundrum_error_variant::ConundrumErrorVariant},
    },
    output::html::web_specific_models::lightning_css_printer_options::safari_specific_lightning_css_printer_options,
};
use lightningcss::{
    properties::PropertyId,
    stylesheet::{ParserOptions, StyleSheet},
    traits::ToCss,
    visit_types,
    visitor::{Visit, Visitor},
};
use winnow::error::ErrMode;

use crate::{errors::DocgenResult, repo::repo_manager::RepoManager};

pub struct ThemeVisitor;

impl<'a> Visitor<'a> for ThemeVisitor {
    type Error = ConundrumError;

    fn visit_color(&mut self, color: &mut lightningcss::values::color::CssColor) -> Result<(), Self::Error> {
        println!("Color: {:#?}", color);
        Ok(())
    }

    fn visit_property(&mut self, property: &mut lightningcss::properties::Property<'a>) -> Result<(), Self::Error> {
        if let Ok(_) = match property.property_id() {
            PropertyId::Custom(c) => match c.to_css_string(safari_specific_lightning_css_printer_options()) {
                Ok(n) => Emphasis::from_css_variable_string(n),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    Err(ErrMode::Cut(ConundrumErrorVariant::DocgenError))
                }
            },
            _ => Err(ErrMode::Cut(ConundrumErrorVariant::DocgenError)),
        } {
            let property_value = property.value_to_css_string(safari_specific_lightning_css_printer_options());
            println!("Property value: {:#?}", property_value);
        } else {
            println!("Failed to load emphasis for property {:#?}",
                     property.property_id()
                             .to_css_string(safari_specific_lightning_css_printer_options())
                             .unwrap_or_default());
        }
        Ok(())
    }

    fn visit_types(&self) -> lightningcss::visitor::VisitTypes {
        visit_types!(PROPERTIES)
    }
}

/// ***Completely*** unfinished.
pub fn write_rust_emphasis_theme_output() -> DocgenResult<()> {
    let manager = RepoManager::new();
    let theme_content = manager.get_themes_css_content();
    let mut styles = StyleSheet::parse(theme_content.as_str(), ParserOptions::default()).expect("Failed to parse themes style sheet to a `StyleSheet` struct.");

    styles.visit(&mut ThemeVisitor).expect("Failed to visit all children in the themes.scss file.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_themes_to_rust_emphasis() {
        write_rust_emphasis_theme_output().expect("Writes rust theme output without throwing an error.");
        println!("This test is completely invalid and will not work. The `write_rust_emphsis_theme_output` method is completely unfinished.")
        // assert_eq!(result, 4);
    }
}
