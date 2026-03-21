use fluster_core_utilities::core_types::component_constants::auto_inserted_component_name::AutoInsertedComponentName;
use serde::Serialize;
use typeshare::typeshare;
use winnow::{ModalResult, Parser, combinator::delimited, token::take_while};

use crate::{
    lang::runtime::traits::{conundrum_input::ConundrumInput, mdx_component_result::MdxComponentResult},
    output::output_components::output_utils::javascript_null_prop,
    parsers::parser_trait::ConundrumParser,
};

#[typeshare]
#[derive(Debug, Serialize)]
pub struct MarkdownLinkResult {
    pub text: String,
    pub url: String,
}

impl MdxComponentResult for MarkdownLinkResult {
    fn to_mdx_component(&self, _: &mut crate::output::parsing_result::mdx_parsing_result::MdxParsingResult) -> String {
        format!("<{} data={{{}}} />",
                AutoInsertedComponentName::AutoInsertedMarkdownLink,
                serde_json::to_string(self).unwrap_or(javascript_null_prop()))
    }
}

impl ConundrumParser<MarkdownLinkResult> for MarkdownLinkResult {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ModalResult<MarkdownLinkResult> {
        let (text, url) =
            (delimited('[', take_while(1.., |c| c != ']' && c != '\n'), ']'),
             delimited('(', take_while(5.., |c| c != ')' && c != '\n' && c != ' ' && c != '\t'), ')'))
                                                                                         .parse_next(input)?;

        Ok(MarkdownLinkResult { text: text.trim().to_string(),
                                url: url.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == '$'
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

        assert!(res.text == "My text here", "Finds the proper text in the markdown link");
        assert!(res.url == "https://google.com",
                "Finds the proper url when the markdown link is simple and formatted properly.")
        // assert_eq!(result, 4);
    }
}
