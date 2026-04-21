use std::sync::Arc;

use serde::Serialize;
use typeshare::typeshare;
use winnow::{Parser, combinator::delimited, token::take_while};

use crate::{
    lang::{
        lib::ui::ui_types::children::Children,
        runtime::{
            parse_conundrum_string::parse_elements,
            state::conundrum_error_variant::ConundrumModalResult,
            traits::{
                conundrum_input::{ArcState, ConundrumInput},
                html_js_component_result::HtmlJsComponentResult,
                mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::{
        general::component_constants::auto_inserted_component_name::AutoInsertedComponentName,
        output_components::output_utils::javascript_null_prop,
    },
    parsers::parser_trait::ConundrumParser,
};

#[typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct MarkdownLinkResult {
    pub text: Children,
    pub url: String,
}

impl PlainTextComponentResult for MarkdownLinkResult {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.text.render(res)
    }
}

impl HtmlJsComponentResult for MarkdownLinkResult {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("<a href=\"{}\">{}</a>", self.url, self.text.render(res)?))
    }
}

impl MdxComponentResult for MarkdownLinkResult {
    fn to_mdx_component(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("<{} data={{{}}} />",
                   AutoInsertedComponentName::AutoInsertedMarkdownLink,
                   serde_json::to_string(self).unwrap_or(javascript_null_prop())))
    }
}

impl ConundrumParser<MarkdownLinkResult> for MarkdownLinkResult {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<MarkdownLinkResult> {
        let (text, url) =
            (delimited('[', take_while(1.., |c| c != ']' && c != '\n'), ']'),
             delimited('(', take_while(5.., |c| c != ')' && c != '\n' && c != ' ' && c != '\t'), ')'))
                                                                                         .parse_next(input)?;

        let state = Arc::clone(&input.state);

        let mut nested_state = ConundrumInput { input: text,
                                                state };
        let res = parse_elements(&mut nested_state)?;

        Ok(MarkdownLinkResult { text: Children(res),
                                url: url.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == '['
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn markdown_text() {
        let test_input = "[My text here](https://google.com)";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownLinkResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        assert!(res.text.render(Arc::clone(&wrapped.state)).is_ok_and(|c| c == "My text here"),
                "Finds the proper text in the markdown link");
        assert!(res.url == "https://google.com",
                "Finds the proper url when the markdown link is simple and formatted properly.")
        // assert_eq!(result, 4);
    }
}
