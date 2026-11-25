
use askama::Template;
use conundrum::lang::runtime::traits::conundrum_template::CDRMTemplateRepresentableWithParam;
use strum::IntoEnumIterator;

#[derive(Template, Default)]
#[template(ext = "md", escape="none", path = "docs/readme.md")]
pub struct DocumentationGenerator {
    pub ctx: crate::generator_context::GeneratorContext
}
