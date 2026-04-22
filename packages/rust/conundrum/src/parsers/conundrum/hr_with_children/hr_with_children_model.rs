use std::sync::Arc;

use askama::Template;
use serde::Serialize;
use winnow::{
    Parser,
    combinator::delimited,
    error::ErrMode,
    stream::Stream,
    token::{literal, take_until},
};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{components::component_trait::ConundrumComponent, ui_types::children::Children},
        runtime::{
            parse_conundrum_string::parse_elements,
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::ConundrumCompileTarget,
            },
            traits::{
                conundrum_input::{ArcState, ConundrumInput},
                fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult,
                markdown_component_result::MarkdownComponentResult,
                mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_names::EmbeddableComponentName,
    },
    parsers::{
        conundrum::{
            hr_with_children::hr_with_children_html_templ::HrWithChildrenHTMLTemplate,
            logic::object::object::ConundrumObject,
        },
        parser_trait::ConundrumParser,
    },
};

#[typeshare::typeshare]
#[derive(Serialize, Debug, Clone)]
pub struct HrWithChildrenResult {
    pub children: Children,
}

impl PlainTextComponentResult for HrWithChildrenResult {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl HtmlJsComponentResult for HrWithChildrenResult {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let templ = HrWithChildrenHTMLTemplate { children: self.children.render(res)? };
        templ.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl ConundrumComponentResult for HrWithChildrenResult {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.compile_target == ConundrumCompileTarget::PlainText {
            drop(state);
            self.to_plain_text(res)
        } else {
            drop(state);
            self.to_mdx_component(res)
        }
    }
}

impl MarkdownComponentResult for HrWithChildrenResult {
    fn to_markdown(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(String::from("---"))
    }
}

impl MdxComponentResult for HrWithChildrenResult {
    // No need to handle this implementation of the to_mdx_component method for the
    // ignore_all_parsers component since children will ignore it.
    fn to_mdx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let children_string = self.children.render(res)?;

        Ok(format!("<Hr>{}</Hr>", children_string))
    }
}

impl ConundrumParser<HrWithChildrenResult> for HrWithChildrenResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<HrWithChildrenResult> {
        let start = input.input.checkpoint();
        let res = delimited(literal("--- "),
                            take_until(1.., " ---").verify(|s: &str| !s.contains("\n")),
                            literal(" ---")).parse_next(input)
                                            .inspect_err(|_| {
                                                input.input.reset(&start);
                                            })?;

        let mut new_input = ConundrumInput { input: res,
                                             state: Arc::clone(&input.state) };
        let elements = parse_elements(&mut new_input)?;

        Ok(HrWithChildrenResult { children: Children(elements) })
    }

    fn matches_first_char(char: char) -> bool {
        char == '['
    }
}

impl ConundrumComponent for HrWithChildrenResult {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::HrWithChildren)
    }

    fn from_props(_: ConundrumObject, children: Option<Vec<ParsedElement>>, _: ArcState) -> ConundrumModalResult<Self>
        where Self: Sized {
        let children = Children(children.unwrap_or_default());
        Ok(HrWithChildrenResult { children })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_hr_with_children() {
        let test_content = "--- My Hr with children ---";
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res = HrWithChildrenResult::parse_input_string(&mut test_data).expect("Parses hr with children without throwing an error.");
        let child = res.to_mdx_component(Arc::clone(&test_data.state)).expect("Compiles to valid mdx");
        assert_snapshot!(child)
        // assert_eq!(result, 4);
    }

    #[test]
    fn parses_hr_with_children_should_fail_with_new_line() {
        let test_content = r#"--- My Hr with 
            children ---"#;
        let mut test_data = wrap_test_conundrum_content(test_content);
        let _ = HrWithChildrenResult::parse_input_string(&mut test_data).expect_err("HR with children parser fails with a new line as it should.");
        // assert_eq!(result, 4);
    }
}
